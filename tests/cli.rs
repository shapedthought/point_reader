use assert_cmd::Command;

#[test]
fn run_all() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
    ]);

    cmd.assert().success();
}

#[test]
fn perf_only_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "p",
    ]);

    cmd.assert().success();
}

#[test]
fn cap_only_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "c",
    ]);

    cmd.assert().success();
}

#[test]
fn arch_only_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "a",
    ]);

    cmd.assert().success();
}

#[test]
fn perf_cap_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "pc",
    ]);

    cmd.assert().success();
}

#[test]
fn perf_arch_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "pa",
    ]);

    cmd.assert().success();
}

#[test]
fn cap_arch_test() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test",
        "-t",
        "ca",
    ]);

    cmd.assert().success();
}

#[test]
fn cannot_find_file() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test2.json",
        "-w",
        "Test",
    ]);

    cmd.assert().failure();
}

#[test]
fn cannot_find_workload() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&[
        "-f",
        "tests\\VSE_test.json",
        "-w",
        "Test2",
    ]);

    cmd.assert().failure();
}
