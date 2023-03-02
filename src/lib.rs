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
}

impl ShellCommand {
    pub fn new(arg0: impl Into<String> + Copy) -> ShellCommand {
        ShellCommand {
            stdin: None,
            argv: vec![arg0.into()],
        }
    }

    pub fn args<I, S>(&self, args: I) -> Self
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
        }
    }

    pub fn pipe_string(self, input: impl Into<String>) -> Self {
        ShellCommand {
            stdin: Some(input.into()),
            argv: self.argv,
        }
    }

    pub fn pipe_stdout(self, scmd: Self) -> Result<Self, String> {
        let scmd_output = match scmd.run() {
            Ok(s) => s,
            Err(s) => return Err(s),
        };
        Ok(self.pipe_string(scmd_output))
    }

    pub fn run(&self) -> Result<String, String> {
        let cmd_arg0 = &self.argv[0];
        let mut cmd = Command::new(cmd_arg0);
        if let Some(s) = &self.stdin {
            let printf = Command::new("printf")
                .arg(s)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process; printf required!")
                .stdout
                .expect("stdout read failed");

            cmd.args(&self.argv[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(printf)
        } else {
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
        };

        let output = cmd.spawn();
        let output = output
            .expect("Failed to spawn child process")
            .wait_with_output()
            .expect("Faled to execute child process");

        if !output.status.success() {
            let stde =
                String::from_utf8(output.stderr).expect("Could not format stderr into utf-8");
            return Err(stde);
        }

        Ok(String::from_utf8(output.stdout).expect("Could not format stdout into utf-8"))
    }

    pub fn command(&self) -> &String {
        &self.argv[0]
    }
}

mod tests;
