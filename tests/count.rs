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
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
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
{\"name\":\"jeff goldblum\",\"list\":[]}
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().success().stdout("3\n");
    }

    #[test]
    fn should_stop_matching_when_max_num_flag_is_specified_and_max_is_reached() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-m").arg("2").arg(r#".name"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"unamed\": null}
{\"name\":\"JEFF goldblum\"}
{\"name\":\"jeff goldblum\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}\n",
        );
    }

    #[test]
    fn should_stop_matching_when_input_end_even_when_max_num_flag_is_specified_as_higher_than_input(
    ) {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-m").arg("5").arg(r#".name"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}
{\"name\":\"jeff goldblum\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"jeff goldblum\"}
{\"name\":\"JEFF goldblum\"}
{\"name\":\"jeff goldblum\"}\n",
        );
    }

    #[test]
    fn should_display_the_line_number_when_line_number_flag_is_specified() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-n").arg(r#".name"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}
{\"unamed\": null}
{\"name\":\"JEFF goldblum\"}
{\"name\":\"jeff goldblum\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "1:{\"name\":\"jeff goldblum\"}
3:{\"name\":\"JEFF goldblum\"}
4:{\"name\":\"jeff goldblum\"}\n",
        );
    }
}
