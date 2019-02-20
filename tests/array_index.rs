#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    use predicates::prelude::*;

    #[test]
    fn should_match_array_with_first_index() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[0]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("[{\"name\":\"inigo montoya\"}]\n");

        assert_cmd.assert().success().stdout("[{\"name\":\"inigo montoya\"}]\n");
    }

    #[test]
    fn should_match_array_with_inbounds_index() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[1]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("[{\"name\":\"inigo montoya\"}]\n
[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]\n");

        assert_cmd.assert().success().stdout("[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]\n");
    }
 
    #[test]
    fn should_not_match_array_with_outofbounds_index() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[2]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("[{\"name\":\"inigo montoya\"}]\n");

        assert_cmd.assert().success().stdout("");
    }

    #[test]
    fn should_match_array_under_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".people[0]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"people\":[{\"name\":\"inigo montoya\"}]}\n");

        assert_cmd.assert().success().stdout("{\"people\":[{\"name\":\"inigo montoya\"}]}\n");
    }
}
