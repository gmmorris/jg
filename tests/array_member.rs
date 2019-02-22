#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_match_array_with_string_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[~="application/javascript"]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\",\"application/javascript\"]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\",\"application/javascript\"]}}\n");
    }

    #[test]
    fn should_match_array_with_exact_string_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[="video/mp4"]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\"]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"mimes\":[\"video/mp4\"]}}\n");
    }

    #[test]
    fn should_match_array_with_a_string_value_which_is_prefixed_by_a_string_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[^="application"]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\"]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}\n");
    }

    #[test]
    fn should_match_array_with_a_string_value_which_is_suffixed_by_a_string_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[$="javascript"]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\"]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}\n");
    }

    #[test]
    fn should_match_array_with_a_string_value_which_is_contained_by_a_string_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[*="tion/jav"]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"mimes\":[\"video/mp4\"]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}\n");
    }

    #[test]
    fn should_match_array_with_exact_boolean_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.opts[=true]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"opts\":[true]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"opts\":[true]}}\n");
    }

    #[test]
    fn should_match_array_with_exact_null_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.opts[=null]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"mimes\":[\"application/javascript\",\"application/x-javascript\",\"video/mp4\"]}}
{\"video\":{\"opts\":[null]}}");

        assert_cmd.assert().success().stdout("{\"video\":{\"opts\":[null]}}\n");
    }

    #[test]
    fn should_match_array_with_numeric_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.sizes[~=480]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"sizes\":[480,640,880]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"sizes\":[480,640,880]}}\n");
    }

    #[test]
    fn should_match_array_with_exact_numeric_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.sizes[=480]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"sizes\":[480,640,880]}}
{\"video\":{\"sizes\":[480]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"sizes\":[480]}}\n");
    }

    #[test] 
    fn should_match_array_with_boolean_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.fields[~=true]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let     mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"fields\":[false,true]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"fields\":[false,true]}}\n");
    }

    #[test]
    fn should_match_array_with_null_value() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.fields[~=null]"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"video\":{\"fields\":[null]}}\n");

        assert_cmd.assert().success().stdout("{\"video\":{\"fields\":[null]}}\n");
    }

    #[test]
    fn should_match_array_with_first_index() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[0]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("[{\"name\":\"inigo montoya\"}]\n");

        assert_cmd
            .assert()
            .success()
            .stdout("[{\"name\":\"inigo montoya\"}]\n");
    }

    #[test]
    fn should_match_array_with_inbounds_index() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("[1]");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(
            "[{\"name\":\"inigo montoya\"}]\n
[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]\n",
        );

        assert_cmd
            .assert()
            .success()
            .stdout("[{\"name\":\"inigo montoya\"},{\"name\":\"John Doe\"}]\n");
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

        assert_cmd
            .assert()
            .success()
            .stdout("{\"people\":[{\"name\":\"inigo montoya\"}]}\n");
    }
}
