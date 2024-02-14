use assert_cmd::Command;

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{self, Path, PathBuf},
    };

    use assert_cmd::Command;

    #[test]
    fn parses_git_log_from_stdin() {
        let test_file = Path::new("tests/data").join("git-log-1");

        let git_log = fs::read_to_string(test_file).unwrap();

        let expected = "Date,Notes\n2021-01-01,Temporary commit\n2021-01-02,Another commit\n";

        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .write_stdin(git_log)
            .assert()
            .success()
            .stdout(expected);
    }
}
