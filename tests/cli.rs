use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_solve"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn original_example_output_is_exact() {
    let output = run(&["365295443"]);
    assert!(output.status.success());
    assert_eq!(output.stdout, b"p:17209\nq:21227\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn no_solution_and_invalid_arguments_have_distinct_statuses() {
    for target in ["0", "1", "2", "7", "97"] {
        let output = run(&[target]);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
    }
    for args in [
        vec![],
        vec!["-1"],
        vec!["hello"],
        vec!["18446744073709551616"],
        vec!["9", "15"],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
    }
}
