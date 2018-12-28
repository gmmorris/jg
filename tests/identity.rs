#[cfg(test)]
mod cli {
    // use std::io::Write;
    use std::process::Command;

    use assert_cmd::prelude::*;
    // use tempfile;

    #[test]
    fn should_match_single_json_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n");

        assert_cmd
            .assert()
            .success()
            .stdout("{}\n");
    }

    #[test]
    fn should_match_all_json_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n{ \"id\": \"404c18ce-04ac-457c-99f5-d548b27aa583\" }\n");

        assert_cmd
            .assert()
            .success()
            .stdout("{}\n{ \"id\": \"404c18ce-04ac-457c-99f5-d548b27aa583\" }\n");
    }

    #[test]
    fn should_not_match_all_when_selector_is_identity() {
        let mut cmd = Command::main_binary().unwrap();

        cmd
            .arg(".prop");
        let mut stdin_cmd = cmd.with_stdin();
        let mut assert_cmd = stdin_cmd.buffer("{}\n");

        assert_cmd
            .assert()
            .success()
            .stdout("");
    }

    // #[test]
    // fn test_run_input_file() {
    //     // the actual example file from Advent of Code

    //     let mut cmd = Command::main_binary().unwrap();

    //     // read in the example input, write to tmp file and run the command
    //     let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
    //     tmp_file.write_all(include_str!("../input/input").as_bytes()).unwrap();

    //     cmd.arg(&tmp_file.path());

    //     cmd
    //         .assert()
    //         .success()
    //         .stdout("Sum of frequencies: 592\n\
    //                  First repeating frequency: 241\n");
    // }

    // #[test]
    // fn test_invalid_num_of_args() {
    //     let mut cmd = Command::main_binary().unwrap();

    //     cmd
    //         .arg("blah")
    //         .arg("blah");

    //     cmd
    //         .assert()
    //         .failure()
    //         .stderr("Error: Invalid number of arguments: 2. Aborting.\n");
    // }
}