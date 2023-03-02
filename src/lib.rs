use std::process::{Command, Stdio};

pub fn about() -> String {
    let str1 = "NoShell is a simple wrapper for std::process::Command;";
    let str2 = "For using rust as a replacement for shell scripting language.";
    format!("{}\n{}", str1, str2)
}

pub struct ShellCommand {
    cmd: Command,
    stdin: Option<Stdio>,
    args: Vec<String>,
}
