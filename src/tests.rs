#[cfg(test)]
use crate::*;

#[test]
fn create_cmd() {
    let a = ShellCommand::new("echo");
    let b = ShellCommand {
        stdin: None,
        argv: vec!["echo".into()],
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

#[test]
fn string_pipe() {
    let sed = ShellCommand::new("sed")
        .args(["-e", "s|dfg|qwerty|g"])
        .pipe_string("asdfghjkl");
    let manual_sed = ShellCommand {
        stdin: Some("asdfghjkl".into()),
        argv: vec!["sed".into(), "-e".into(), "s|dfg|qwerty|g".into()],
    };

    assert_eq!(sed, manual_sed);
    assert_eq!(sed.run().unwrap(), manual_sed.run().unwrap());
}

#[test]
fn simple_run() {
    let sed = ShellCommand::new("sed")
        .args(["-e", "s|dfg|qwerty|g"])
        .pipe_string("asdfghjkl")
        .run()
        .unwrap();
    assert_eq!("asqwertyhjkl", sed);
}

#[test]
fn multiple_run() {
    let sed = ShellCommand::new("sed")
        .args(["-e", "s|dfg|qwerty|g"])
        .pipe_string("asdfghjkl");

    let ans1 = sed.run().unwrap();
    let ans2 = sed.run().unwrap();

    assert_eq!(ans1, ans2);
}

#[test]
fn get_stderr() {
    let sed_err = match ShellCommand::new("sed").run() {
        Ok(_) => "",
        Err(_) => "Error detected",
    };

    assert_eq!(sed_err, "Error detected");
}

#[test]
fn cmd_pipe() {
    let sed = ShellCommand::new("sed")
        .args(["-e", "s|dfg|qwerty|g"])
        .pipe_string("asdfghjkl");

    let sed2 = ShellCommand::new("sed")
        .args(["-e", "s|as|qwerty|g"])
        .pipe_stdout(sed)
        .unwrap()
        .run()
        .unwrap();

    assert_eq!("qwertyqwertyhjkl", sed2);
}
