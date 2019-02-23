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
{\"name\":\"inigo montoya\",\"list\":[]}
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
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
{\"name\":\"inigo montoya\",\"list\":[]}
{\"list\":[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd.assert().failure().code(predicate::eq(1));
    }
}
