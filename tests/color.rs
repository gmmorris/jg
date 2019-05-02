#[cfg(test)]
mod cli {
    use std::process::Command;

    use assert_cmd::prelude::*;
    use colored::*;

    #[test]
    fn should_color_matching_json() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("--color");
        cmd.arg(".name");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"name\":\"jeff goldblum\"}\n");

        assert_cmd
            .assert()
            .success()
            .stdout(
                format!(
                    r#"{}{}{}
"#,
                    r#"{"name":"#,
                    r#""jeff goldblum""#.red(),
                    r#"}"#
                ) 
            );
    }

    #[test]
    fn should_color_multiple_matching_json() {
        let mut cmd = Command::main_binary().unwrap();

        cmd.arg("--color");
        cmd.arg("-e").arg(r#".name"#);
        cmd.arg("-e").arg(r#".middle_name"#);

        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{\"name\":\"jeff goldblum\",\"middle_name\":\"lynn\"}\n");

        assert_cmd
            .assert()
            .success()
            .stdout(
                format!(
                    r#"{}{}{}{}{}
"#,
                    r#"{"name":"#,
                    r#""jeff goldblum""#.red(),
                    r#","middle_name":"#,
                    r#""lynn""#.blue(),
                    r#"}"#
                ) 
            );
    }

}
