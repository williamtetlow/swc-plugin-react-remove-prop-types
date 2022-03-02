use std::{path::Path, process::Command};

/// Runs `npx swc` command with plugin in swcrc config
fn main() {
    let output = Command::new("npm")
        .arg("run")
        .arg("example:usage")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
        .output()
        .expect("failed to run `npm run example:usage`");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}
