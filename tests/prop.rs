#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    use predicates::prelude::*;

    #[test]
    fn should_match_single_json_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"name\":\"inigo montoya\"}\n");

        assert_cmd.assert().success().stdout("{\"name\":\"inigo montoya\"}\n");
    }

    #[test]
    fn should_match_only_json_with_prop_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{ \"name\":\"inigo montoya\" }
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"inigo montoya\"}\n",
        );
    }

    #[test]
    fn should_match_json_with_deep_matching_props() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".job.title");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}
{\"name\":\"John Doe\",\"job\":{}}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}\n",
        );
    }

    #[test]
    fn should_match_json_with_inner_matching_props() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".job.title");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}
{\"name\":\"John Doe\",\"title\":\"mr\"}
{\"name\":\"John Doe\",\"self\":{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}
{\"name\":\"John Doe\",\"self\":{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}}\n",
        );
    }

    #[test]
    fn should_match_multiple_json_with_matching_prop_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{ \"name\":\"inigo montoya\" }
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{ \"name\":\"blanco white\" }\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"inigo montoya\"}\n{\"name\":\"blanco white\"}\n",
        );
    }
}
