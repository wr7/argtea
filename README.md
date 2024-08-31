Declarative macro commandline parser (inspired by `argwerk`).

argtea attempts to reduce abstraction and maximize flexibility.

## Comparison to argwerk

|                                      | `argtea`  | `argwerk` |
| :----------------------------------- | :-------: | :-------: |
| Boilerplate                          | More      | Less      |
| `--flag=value` syntax                | Yes       | No        |
| `-sw 80` <=> `-s -w 80` syntax       | Yes       | No        |
| OsString argument support            | No        | Yes       |
| Customizable help message formatting | Yes       | Yes*      |
| Help message generation              | Yes       | Yes*      |

[*] At runtime

## Example project
```rust
use argtea::{argtea_impl, simple_format};

#[derive(Debug)]
pub struct Arguments {
    output_path: String,
    files: Vec<String>,
}

fn main() -> Result<(), String> {
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
            if file.starts_with("-") {
                return Err(format!("invalid flag `{file}`"));
            }
            
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

        fn parse() -> Result<Self, String> {
            let mut files = Vec::new();
            let mut output_path_ = None;

            parse!(std::env::args().skip(1));

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
