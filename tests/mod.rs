
use std::process::Command;

#[test]
fn test_run() {
    let path = format!("target/debug/{}", env!("CARGO_PKG_NAME"));

    match Command::new(path).output() {
        Ok(cmd) => {
            assert!(cmd.status.success());
        },
        Err(err) => {
            panic!(err);
        }
    }
}


