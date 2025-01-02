use crate::argtea_impl;

struct TestA {
    pub warning: Option<String>,
}

argtea_impl! {
    {
        /// This will not be visible in the documentation
        #[hidden]
        /// Neither will this
        ("-a") => {
            println!("a");
        }

        /// This flag does not do anything and does not show up in documentation
        #[fake]
        #[hidden]
        ("--fake-and-hidden") => {
            compile_error!("Fake flags should not be compiled");
        }

        /// Enables all warnings
        #[fake]
        ("-Wall") => {}

        /// Enables a specific warning
        ///
        /// b
        ("--warning" | "-W", warning) => {
            let Some(warning) = warning else {
                panic!("expected parameter for warning flag")
            };

            warning_ = Some(warning);
        }

        (flag_name @ "--weird-flag" | "-f", p) => {
            let Some(p) = p else {
                panic!("expected parameter for `{flag_name}`")
            };

            warning_ = Some(format!("{flag_name} {p}"));
        }

        (other) => {
            panic!("unexpected parameter `{other}`");
        }
    }

    impl TestA {
        #[allow(unused)]
        pub const DOCS: &'static str = crate::simple_format!(
            "argtea test A"
            docs!()
            "end"
        );

        /// H
        #[export_name = "TestA_parse"]
        extern "Rust" fn parse(params: Vec<String>) -> TestA {
            let mut warning_ = None;

            parse!(params.into_iter());

            TestA {warning: warning_}
        }

        /// H2
        #[export_name = "TestA_parse2"]
        pub(self) extern "Rust" fn parse2() {}
    }
}

#[test]
fn test_a() {
    let tests: &[(&[&str], &str)] = &[
        (&["-W", "all"], "all"),
        (&["-aW", "all"], "all"),
        (&["--warning", "all"], "all"),
        (&["--warning=all"], "all"),
        (&["-Wall"], "all"),
        (&["-aWall"], "all"),
        (&["-f", "abc"], "-f abc"),
        (&["-fabc"], "-f abc"),
        (&["--weird-flag", "def"], "--weird-flag def"),
    ];

    for (test, expected) in tests {
        let args: Vec<String> = test.iter().map(|a| a.to_string()).collect();

        let result = TestA::parse(args);
        assert_eq!(result.warning.as_deref(), Some(*expected));
    }
}

#[test]
fn test_a_docs() {
    const EXPECTED: &'static str = include_str!("tests/test_a.txt");

    assert_eq!(TestA::DOCS, EXPECTED);
}
