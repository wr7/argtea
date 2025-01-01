use crate::argtea_impl;

struct TestA {
    pub warning: Option<String>,
}

argtea_impl! {
    {
        #[hidden]
        ("-a") => {
            println!("a");
        }

        ("--warning" | "-W", warning) => {
            let Some(warning) = warning else {
                panic!("expected parameter for `-W`")
            };

            warning_ = Some(warning);
        }

        (other) => {
            panic!("unexpected parameter `{other}`");
        }
    }

    impl TestA {
        #[allow(unused)]
        pub(self) const DOCS: &'static str = crate::simple_format!(docs!());

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
    let tests: &[&[&str]] = &[
        &["-W", "all"],
        &["-aW", "all"],
        &["--warning", "all"],
        &["--warning=all"],
        &["-Wall"],
        &["-aWall"],
    ];

    for test in tests {
        let args: Vec<String> = test.iter().map(|a| a.to_string()).collect();

        let result = TestA::parse(args);
        assert_eq!(result.warning.as_deref(), Some("all"));
    }
}
