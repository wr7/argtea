Declarative macro commandline parser (inspired by argwerk).

The goal of argtea (pronounced arg tea) is to automatically generate help
pages using doc comments. argtea attempts to be more flexible and less
abstracted than argwerk.

Example project:
```rust
use argtea::{argtea_impl, simple_format};

#[derive(Debug)]
pub struct Arguments {
    output_path: String,
    files: Vec<String>,
}

fn main() -> Result<(), &'static str> {
    let args = Arguments::parse()?;

    println!("input files: {:?}", args.files);
    println!("output file: {:?}", args.output_path);

    Ok(())
}

argtea_impl! {
    {
        /// Displays this help message.
        ("--help" | "-h") => {
            eprintln!("{}", Self::HELP);

            std::process::exit(0);
        }

        /// Sets the output file path.
        ("--output" | "-o", output_path) => {
            output_path_ = output_path;
        }

        /// Adds a file as an input.
        ///
        /// To input a file that starts with a `-`, prefix it with a `./`
        (file) => {
            files.push(file);
        }
    }

    impl Arguments {
        const HELP: &'static str = simple_format!(
            "argtea_test: a demo argtea project"
            ""
            "Usage: "
            "  `argtea_test [FLAGS] [FILES]`"
            ""
            "Options:"
            docs!()
        );

        fn parse() -> Result<Self, &'static str> {
            let mut args = std::env::args().skip(1);

            let mut files = Vec::new();
            let mut output_path_ = None;

            parse!(args);

            return Ok(Self {
                files,
                output_path: output_path_.unwrap_or_else(|| "a.out".to_owned())
            });
        }
    }
}
```

output from `argtea_test -h`:
```
argtea_test: a demo argtea project

Usage:
  `argtea_test [FLAGS] [FILES]`

Options:
  --help, -h
    Displays this help message.


  --output, -o <output_path>
    Sets the output file path.


  <file>
    Adds a file as an input.

    To input a file that starts with a `-`, prefix it with a `./`

```
