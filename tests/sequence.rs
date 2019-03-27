#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_match_array_with_anything_in_it_when_identity_is_used() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "[{\"name\":\"jeff goldblum\"}]\n
{\"name\":\"jeff goldblum\"}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("[{\"name\":\"jeff goldblum\"}]\n");
    }

    #[test]
    fn should_match_array_with_null_in_it() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "[null]\n
{\"name\":\"jeff goldblum\"}\n",
        );

        assert_cmd.assert().success().stdout("[null]\n");
    }

    #[test]
    fn should_match_array_under_prop_containing_anything_when_selecting_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}\n
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n");
    }

    #[test]
    fn should_match_member_in_array_when_selecting_by_prop() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.name]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\",\"list\":[]}\n
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n");
    }

    #[test]
    fn should_match_members_in_array_when_root_of_pattern_is_within_the_array() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#"{"name":"John Doe"}"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            r#"{"name":"jeff goldblum","list":[]}
{"name":"John Doe"}
{"list":[{"name":"jeff goldblum"},{"name":"John Doe"}]}"#,
        );

        assert_cmd.assert().success().stdout(
            r#"{"name":"John Doe"}
{"list":[{"name":"jeff goldblum"},{"name":"John Doe"}]}
"#,
        );
    }

    #[test]
    fn should_match_array_patterns_within_arrays() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".friends[.]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            r#"{"name":"John Doe","friends":[]}
{"list":[{"name":"jeff goldblum","friends":[{"name":"Fezzik"}]},{"name":"John Doe"}]}"#,
        );

        assert_cmd.assert().success().stdout(
            r#"{"list":[{"name":"jeff goldblum","friends":[{"name":"Fezzik"}]},{"name":"John Doe"}]}
"#,
        );
    }

    #[test]
    fn should_not_match_member_in_array_when_prop_matches_deep_object_in_member() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[{\"name\":\"jeff goldblum\"}]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\",\"list\":[]}\n
{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n
{\"list\":[{\"name\":\"John Doe\",\"father\":{\"name\":\"jeff goldblum\"}}]}\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("{\"list\":[{\"name\":\"jeff goldblum\"},{\"name\":\"John Doe\"}]}\n");
    }

    #[test]
    fn should_match_sequence_matchers_within_other_sequences() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".bid_request.imp[0].pmp.deals[{"id":"BIDDER-DEAL-1"}]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"bid_request\":{\"imp\":[{\"pmp\":{\"deals\":[{\"id\":\"BIDDER-DEAL-1\"}],\"private_auction\":0}}]}}\n");

        assert_cmd.assert().success().stdout("{\"bid_request\":{\"imp\":[{\"pmp\":{\"deals\":[{\"id\":\"BIDDER-DEAL-1\"}],\"private_auction\":0}}]}}\n");
    }

    #[test]
    fn should_not_match_empty_array_when_selecting_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(".list[.]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "{\"name\":\"jeff goldblum\"}\n
{\"list\":[]}\n",
        );

        assert_cmd.assert().failure();
    }
}
