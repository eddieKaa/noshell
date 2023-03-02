#[cfg(test)]
use crate::*;

#[test]
fn create_cmd() {
    let a = ShellCommand::new("echo");
    let b = ShellCommand {
        stdin: None,
        argv: vec!["echo".into()],
        output: None,
    };

    assert_eq!(a, b)
}

#[test]
fn add_args() {
    let a = ShellCommand::new("echo").args(&["Hello", "World!"]);
    let b = ShellCommand::new("echo").args(&["Hello".to_string(), "World!".to_string()]);
    let c = ShellCommand::new("echo").args(vec!["Hello", "World!"]);
    let d = ShellCommand::new("echo").args(vec!["Hello".to_string(), "World!".to_string()]);

    assert_eq!(a, b);
    assert_eq!(c, d);
    assert_eq!(b, d);
}
