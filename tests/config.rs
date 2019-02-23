#[cfg(test)]
mod cli {
    // use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;
    // use tempfile;

    #[test]
    fn should_print_out_count_when_c_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-c");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"inigo montoya\",\"list\":[]}
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().success().stdout("3\n");
    }

    #[test]
    fn should_print_out_count_when_count_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("--count");

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{}
{\"name\":\"inigo montoya\",\"list\":[]}
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().success().stdout("3\n");
    }
}
