use swc_plugin::{ast::*, plugin_transform, util::take::Take};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_member_expr(&mut self, mem_expr: &mut MemberExpr) {
        match &mem_expr.prop {
            MemberProp::Ident(id) => {
                if &*id.sym == "propTypes" {
                    mem_expr.take();
                }
            }
            _ => (),
        }
    }

    fn visit_mut_pat(&mut self, pat: &mut Pat) {
        pat.visit_mut_children_with(self);

        if let Pat::Expr(expr) = pat {
            if let Expr::Member(member) = &**expr {
                if let Expr::Invalid(_) = &*member.obj {
                    pat.take();
                }
            }
        }
    }

    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_mut_children_with(self);

        if let Stmt::Expr(ExprStmt { expr, .. }) = stmt {
            if let Expr::Assign(assign) = &**expr {
                if let PatOrExpr::Pat(pat) = &assign.left {
                    if let Pat::Invalid(_) = &**pat {
                        stmt.take();
                    }
                }
            }
        }
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        stmts.retain(|s| !matches!(s, ModuleItem::Stmt(Stmt::Empty(..))));
    }
}

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually via
/// `__plugin_process_impl(
///     ast_ptr: *const u8,
///     ast_ptr_len: i32,
///     config_str_ptr: *const u8,
///     config_str_ptr_len: i32,
///     context_str_ptr: *const u8,
///     context_str_ptr_len: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */
///
/// if plugin need to handle low-level ptr directly. However, there are
/// important steps manually need to be performed like sending transformed
/// results back to host. Refer swc_plugin_macro how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _plugin_config: String, _context: String) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
