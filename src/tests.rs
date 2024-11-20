use crate::argtea_impl;

struct TestA {
    pub warning: Option<String>,
}

argtea_impl! {
    {
        ("-a") => {}

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
        fn parse(params: Vec<String>) -> TestA {
            let mut warning_ = None;

            parse!(params.into_iter());

            return TestA {warning: warning_};
        }
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
