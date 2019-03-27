#[cfg(test)]
mod cli {
    // use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;
    // use tempfile;

    #[test]
    fn should_search_with_case_sensitivity_on_by_default() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name":"jeff goldblum"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}
{\"NAME\":\"jeff goldblum\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"jeff goldblum\"}\n");
    }

    #[test]
    fn should_search_with_case_sensitivity_switched_off_when_the_i_flag_is_used() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-i").arg(r#"{"name":"jeff goldblum"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}
{\"NAME\":\"jeff goldblum\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}
{\"NAME\":\"jeff goldblum\"}\n",
        );
    }
}
