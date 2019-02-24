#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_match_single_json_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"name\":\"inigo montoya\"}\n");

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"inigo montoya\"}\n");
    }

    #[test]
    fn should_match_only_root_of_json_when_selector_is_prop_with_start_matcher() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("-^").arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"name\":\"inigo montoya\"}
{\"person\":{\"name\":\"John Doe\"}}");

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"inigo montoya\"}\n");
    }

    #[test]
    fn should_match_only_json_with_prop_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"inigo montoya\"}\n");
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

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown\"}}\n");
    }

    #[test]
    fn should_match_multiple_json_with_matching_prop_when_selector_is_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"inigo montoya\"}\n{\"name\":\"blanco white\"}\n");
    }

    #[test]
    fn should_match_json_porperty_using_dictionary_index_selector() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"inigo montoya\"}
{\"name\":\"blanco white\"}\n",
        );
    }

    #[test]
    fn should_match_json_porperty_with_a_value_when_using_the_exact_matcher() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name":"blanco white"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"blanco white\"}\n");
    }

    #[test]
    fn should_match_json_porperty_with_a_value_when_using_the_contains_exact_matcher() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name"~:"white"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"name\":\"blanco white\"}\n");
    }

    #[test]
    fn should_match_json_porperty_with_a_value_when_using_the_contains_matcher() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name"*:"o "}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"inigo montoya\"}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}
{\"name\":\"blanco white\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"inigo montoya\"}
{\"name\":\"blanco white\"}\n",
        );
    }

    #[test]
    fn should_match_json_with_inner_matching_props() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".job{"title":"Unknown-title"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown-title\"}}
{\"name\":\"John Doe\",\"title\":\"mr\"}
{\"name\":\"John Doe\",\"self\":{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown-title\"}}}
{\"id\":\"404c18ce-04ac-457c-99f5-d548b27aa583\"}\n",
        );

        assert_cmd.assert().success().stdout(
            "{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown-title\"}}
{\"name\":\"John Doe\",\"self\":{\"name\":\"John Doe\",\"job\":{\"title\":\"Unknown-title\"}}}\n",
        );
    }
}
