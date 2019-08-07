#[cfg(test)]
mod parameters {
    use std::process::Command;

    use assert_cmd::prelude::*;

    #[test]
    fn should_substitute_a_pair_of_braces_with_a_single_parameter() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[*="{}"]"#);
        cmd.arg("--params").arg(r#"mp4"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(r#"{"video":{"mimes":["application/x-shockwave-flash","audio/mp4","image/jpg","image/gif"]}}
"#);

        assert_cmd
            .assert()
            .success()
            .stdout(r#"{"video":{"mimes":["application/x-shockwave-flash","audio/mp4","image/jpg","image/gif"]}}
"#);
    }

    #[test]
    fn should_substitute_across_multiple_patters_in_order() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[*="{}"]"#);
        cmd.arg("-e").arg(r#".video.mimes[*="{}"]"#);
        cmd.arg("-e").arg(r#".audio{"format":"{}"}"#);
        cmd.arg("--params").arg(r#"mp4"#);
        cmd.arg("--params").arg(r#"aac"#);
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(r#"{"video":{"mimes":["audio/mp4","image/jpg","image/gif"]}}
{"audio":{"format":"aac"}}
{"audio":{"format":"{}"}}
"#);

        assert_cmd
            .assert()
            .success()
            .stdout(r#"{"video":{"mimes":["audio/mp4","image/jpg","image/gif"]}}
{"audio":{"format":"aac"}}
"#);
    }

    #[test]
    fn should_substitute_only_the_first_pair_of_braces_with_a_single_parameter() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg(r#".video.mimes[*="{}"]"#);
        cmd.arg("-e").arg(r#".video.mimes[*="{}"]"#);
        cmd.arg("-e").arg(r#".audio{"format"*:"{}"}"#);
        cmd.arg("--params").arg(r#"mp4"#);
        
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer(r#"{"video":{"mimes":["audio/mp4","image/jpg","image/gif"]}}
{"audio":{"format":"aac"}}
{"audio":{"format":"---{}---"}}
"#);

        assert_cmd
            .assert()
            .success()
            .stdout(r#"{"video":{"mimes":["audio/mp4","image/jpg","image/gif"]}}
{"audio":{"format":"---{}---"}}
"#);
    }
}
