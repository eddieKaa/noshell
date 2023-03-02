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

pub struct CommandResult {
    output: Result<String, String>,
}

impl ShellCommand {
    pub fn new(arg0: impl Into<String> + Copy) -> ShellCommand {
        let mut cmd = Command::new(arg0.into());
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        ShellCommand {
            cmd,
            stdin: None,
            args: vec![arg0.into()],
        }
    }

    fn command(&self) -> &String {
        &self.args[0]
    }
}
