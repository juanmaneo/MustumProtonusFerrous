use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("mpf-proton-cli-integration-{nonce}"))
}

#[test]
fn binary_commands_and_errors() {
    let dir = temp_dir();
    std::fs::create_dir_all(&dir).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .args(["env", "prefix"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("WINEPREFIX=prefix"));

    let conf = dir.join("dxvk.conf");
    std::fs::write(&conf, "dxvk.hud = 1\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .args(["env", "prefix", "--dxvk-conf"])
        .arg(&conf)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));

    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .args(["env", "prefix", "--dxvk-conf"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));

    let missing_conf = dir.join("missing.conf");
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .args(["env", "prefix", "--dxvk-conf"])
        .arg(missing_conf)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));

    let prefix = dir.join("prefix");
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .arg("setup")
        .arg(&prefix)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));

    let exe = dir.join("game.exe");
    std::fs::write(&exe, mpf_pe::testing::build_test_pe64()).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .arg("run")
        .arg(&prefix)
        .arg(&exe)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("entry point:"));

    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .args(["run", "prefix", "missing.exe"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));

    let blocked = dir.join("not-a-directory");
    std::fs::write(&blocked, b"file").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .arg("setup")
        .arg(blocked)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));

    let output = Command::new(env!("CARGO_BIN_EXE_mpf-proton"))
        .current_dir(&dir)
        .arg("bogus")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));

    std::fs::remove_dir_all(dir).unwrap();
}
