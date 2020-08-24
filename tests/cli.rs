extern crate assert_cli;

#[cfg(test)]
mod cli {
    use assert_cli;

    #[test]
    #[ignore]
    fn invalid_argument() {
        assert_cli::Assert::main_binary()
            .with_args(&["-Z"])
            .fails()
            .and()
            .stderr()
            .contains("error: Found argument '-Z' which wasn't expected")
            .unwrap();
    }

    #[test]
    fn help_works() {
        assert_cli::Assert::main_binary()
            .with_args(&["-h"])
            .succeeds()
            .and()
            .stdout()
            .contains("tracker")
            .unwrap();
    }

    #[test]
    fn start_and_stop_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["start", "-t", "task1", "-p", "myproject"])
            .succeeds()
            .and()
            .stdout()
            .contains("Starting")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["start", "-t", "task2", "-p", "myproject2"])
            .succeeds()
            .and()
            .stdout()
            .contains("running")
            .unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&["stop"])
            .succeeds()
            .and()
            .stdout()
            .contains("Stopping")
            .unwrap();
    }
    
    #[test]
    fn status_timer() {
        assert_cli::Assert::main_binary()
            .with_args(&["status"])
            .succeeds()
            .and()
            .stdout()
            .contains("running")
            .unwrap();
    }

    #[test]
    fn print_hours() {
        assert_cli::Assert::main_binary()
            .with_args(&["hours"])
            .succeeds()
            .and()
            .stdout()
            .contains("hours")
            .unwrap();
    }

    #[test]
    fn print_weeks() {
        assert_cli::Assert::main_binary()
            .with_args(&["weeks"])
            .succeeds()
            .and()
            .stdout()
            .contains("Week")
            .unwrap();
    }

    #[test]
    #[ignore]
    fn print_days() {
        assert_cli::Assert::main_binary()
            .with_args(&["days"])
            .succeeds()
            .and()
            .stdout()
            .contains("Day")
            .unwrap();
    }

    #[test]
    fn cancel_timers() {
        assert_cli::Assert::main_binary()
            .with_args(&["cancel"])
            .succeeds()
            .unwrap();
    }

}
