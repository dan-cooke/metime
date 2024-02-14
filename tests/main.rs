use assert_cmd::Command;

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    fn parses_git_log_from_stdin() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(&["-"])
            .write_stdin("commit 1\ncommit 2\n")
            .assert()
            .success()
            .stdout("commit 1\ncommit 2\n"
    }
    
}
