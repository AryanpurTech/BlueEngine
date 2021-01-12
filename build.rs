use std::process::Command;

#[cfg(windows)]
fn main() {
    Command::new("set PATH=%PATH%;%CD%\\dep\\ninja");
}
