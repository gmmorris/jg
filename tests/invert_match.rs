#[cfg(test)]
mod cli {
    // use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;
    // use tempfile;

    #[test]
    fn should_invert_match_when_v_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-v").arg(r#"{"name":"jeff goldblum"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );
    }

    #[test]
    fn should_invert_match_when_invert_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("--invert-match").arg(r#"{"name":"jeff goldblum"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );
    }
}
