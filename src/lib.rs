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
        let str_fi = |s: S| s.to_string();
        let arg_addition: Vec<String> = args.into_iter().map(str_fi).collect();

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
            cmd.args(&self.argv[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(string_to_stdio(s))
        } else {
            cmd.stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
        };

        let output = cmd.spawn();
        let output = output.expect(EXP_SP).wait_with_output().expect(EXP_EXE);

        if !output.status.success() {
            let stde = String::from_utf8(output.stderr).expect(EXP_SERR);
            return Err(stde);
        }

        Ok(String::from_utf8(output.stdout).expect(EXP_SOUT))
    }

    pub fn command(&self) -> &String {
        &self.argv[0]
    }
}

fn string_to_stdio(s: impl Into<String>) -> Stdio {
    Command::new("printf")
        .arg(s.into())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process; printf required!")
        .stdout
        .expect("stdout read failed")
        .into()
}

static EXP_SOUT: &str = "Could not format stdout into utf-8";
static EXP_SERR: &str = "Could not format stderr into utf-8";
static EXP_SP: &str = "Failed to spawn child process";
static EXP_EXE: &str = "Faled to execute child process";
mod tests;
