#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;
    use predicates::prelude::*;

    #[test]
    fn should_return_exitcode_0_when_matches_are_found() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-c").arg(".name");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().success().code(predicate::eq(0));
    }

    #[test]
    fn should_return_exitcode_1_when_no_matches_are_found() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-c").arg(".age");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().failure().code(predicate::eq(1));
    }

    #[test]
    fn should_only_return_exitcode_when_a_match_is_found_and_quiet_mode_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-q").arg(".name");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .code(predicate::eq(0))
            .stdout("");
    }

    #[test]
    fn should_only_return_exitcode_when_no_matches_are_found_and_quiet_mode_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-q").arg(".age");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .failure()
            .code(predicate::eq(1))
            .stdout("");
    }

    #[test]
    fn should_match_multiple_patterns_when_multiple_patterns_are_provided() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-e").arg(r#"{"eye_color":"yellow"}"#);
        cmd.arg("-e").arg(r#"{"hair_color":"n/a"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            r#"{"name":"Luke Skywalker","hair_color":"blond","eye_color":"blue"}
{"name":"C-3PO","hair_color":"n/a","eye_color":"yellow"}
{"name":"R2-D2","hair_color":"n/a","eye_color":"red"}
{"name":"Admiral Ackbar","hair_color":"n/a","eye_color":"yellow"}
{"name":"Obi-Wan Kenobi","hair_color":"auburn, white","eye_color":"blue-gray"}"#,
        );

        assert_cmd.assert().success().stdout(
            r#"{"name":"C-3PO","hair_color":"n/a","eye_color":"yellow"}
{"name":"R2-D2","hair_color":"n/a","eye_color":"red"}
{"name":"Admiral Ackbar","hair_color":"n/a","eye_color":"yellow"}
"#,
        );
    }
}
