use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("mpf-umu-cli-integration-{nonce}"))
}

#[test]
fn binary_config_and_error_paths() {
    let dir = temp_dir();
    std::fs::create_dir_all(&dir).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-umu"))
        .current_dir(&dir)
        .env("HOME", &dir)
        .env("UMU_RUNTIME", "sniper")
        .args(["config"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(0));
    assert!(stdout.contains("UMU_RUNTIME=sniper"));
    assert!(stdout.contains("WINEPREFIX="));

    let config = dir.join("config.toml");
    std::fs::write(&config, "[umu]\nruntime = soldier\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_mpf-umu"))
        .current_dir(&dir)
        .env("HOME", &dir)
        .args(["config", "--config"])
        .arg(&config)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("UMU_RUNTIME=soldier"));

    let output = Command::new(env!("CARGO_BIN_EXE_mpf-umu"))
        .current_dir(&dir)
        .args(["config", "--config"])
        .arg(&dir)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));

    std::fs::remove_dir_all(dir).unwrap();
}
