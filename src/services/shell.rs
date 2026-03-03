use std::env;
use std::env::home_dir;
use std::process::Command;

pub fn get_terminal_cfg_path() -> String {
    terminal_cfg_source_path()
        .replace("~", &home_dir().unwrap().to_str().unwrap())
}

pub fn reload_shell() {
    let source = terminal_cfg_source_path();
    let command = format!("source {}", source);

    Command::new("bash")
        .arg("-c")
        .arg("-i")
        .arg(command)
        .spawn()
        .expect("Failed to reload shell config");
}

fn terminal_cfg_source_path() -> String {
    env::var("PATH_TO_TERMINAL_CFG")
        .expect("$PATH_TO_TERMINAL_CFG is not set")
}