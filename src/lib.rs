use std::process::{Command, Stdio};

pub fn about() -> String {
    let str1 = "NoShell is a simple wrapper for std::process::Command;";
    let str2 = "For using rust as a replacement for shell scripting language.";
    format!("{}\n{}", str1, str2)
}

#[derive(Debug, PartialEq)]
pub struct ShellCommand {
    stdin: Option<String>,
    argv: Vec<String>,
    output: Option<Result<String, String>>,
}

impl ShellCommand {
    pub fn new(arg0: impl Into<String> + Copy) -> ShellCommand {
        ShellCommand {
            stdin: None,
            argv: vec![arg0.into()],
            output: None,
        }
    }

    pub fn args<I, S>(&self, args: I) -> ShellCommand
    where
        I: IntoIterator<Item = S>,
        S: std::fmt::Display,
    {
        let arg_addition: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();

        let mut argv = self.argv.clone();
        argv.extend(arg_addition);

        ShellCommand {
            stdin: self.stdin.clone(),
            argv,
            output: self.output.clone(),
        }
    }

    pub fn command(&self) -> &String {
        &self.argv[0]
    }
}

mod tests;
