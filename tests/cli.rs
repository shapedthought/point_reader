use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&["-f", "tests\\VSE_Result_15_04_2023, 14_59_30.json", "-w", "Test"]);

    cmd.assert().success();

}

#[test]
fn cannot_find_file() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&["-f", "tests\\VSE_Result_15_04_2023, 14_59_31.json", "-w", "Test"]);

    cmd.assert().failure();

}

#[test]
fn cannot_find_workload() {
    let mut cmd = Command::cargo_bin("point_reader").unwrap();

    cmd.args(&["-f", "tests\\VSE_Result_15_04_2023, 14_59_30.json", "-w", "Test2"]);

    cmd.assert().failure();

}