//! Declarative macro commandline parser (inspired by argwerk).
//!
//! argtea attempts to reduce abstraction and maximize flexibility.
//!
//! ## Comparison to argwerk
//!
//! |                                      | `argtea`  | `argwerk` |
//! | :----------------------------------- | :-------: | :-------: |
//! | Boilerplate                          | More      | Less      |
//! | `--flag=value` syntax                | Yes       | No        |
//! | `-sw 80` <=> `-s -w 80` syntax       | Yes       | No        |
//! | OsString argument support            | No        | Yes       |
//! | Customizable help message formatting | Yes       | Yes*      |
//! | Help message generation              | Yes       | Yes*      |
//!
//! [*] At runtime
//!
//! ## Example project:
//! ```rust
//! use argtea::{argtea_impl, simple_format};
//!
//! #[derive(Debug)]
//! pub struct Arguments {
//!     output_path: String,
//!     files: Vec<String>,
//! }
//!
//! fn main() -> Result<(), String> {
//!     let args = Arguments::parse()?;
//!
//!     println!("input files: {:?}", args.files);
//!     println!("output file: {:?}", args.output_path);
//!
//!     Ok(())
//! }
//!
//! argtea_impl! {
//!     {
//!         /// Displays this help message.
//!         ("--help" | "-h") => {
//!             eprintln!("{}", Self::HELP);
//!
//!             std::process::exit(0);
//!         }
//!
//!         /// Sets the output file path.
//!         ("--output" | "-o", output_path) => {
//!             output_path_ = output_path;
//!         }
//!
//!         /// Adds a file as an input.
//!         ///
//!         /// To input a file that starts with a `-`, prefix it with a `./`
//!         (file) => {
//!             if file.starts_with("-") {
//!                 return Err(format!("invalid flag `{file}`"));
//!             }
//!             
//!             files.push(file);
//!         }
//!     }
//!
//!     impl Arguments {
//!         const HELP: &'static str = simple_format!(
//!             "argtea_test: a demo argtea project"
//!             ""
//!             "Usage: "
//!             "  `argtea_test [FLAGS] [FILES]`"
//!             ""
//!             "Options:"
//!             docs!()
//!         );
//!
//!         fn parse() -> Result<Self, String> {
//!             let mut files = Vec::new();
//!             let mut output_path_ = None;
//!
//!             parse!(std::env::args().skip(1));
//!
//!             return Ok(Self {
//!                 files,
//!                 output_path: output_path_.unwrap_or_else(|| "a.out".to_owned())
//!             });
//!         }
//!     }
//! }
//! ```
//!
//! # Footguns
//! Because argtea only uses declarative macros, it is somewhat limited and has
//! a few footguns.
//!
//! ## Visibility
//! Visibility cannot be specified inside of [`argtea_impl`] invokations.
//! The following will NOT compile because `FLAGS` and `parse` are both
//! declared as `pub`:
//! ```compile_fail
//! # use argtea::argtea_impl;
//! # pub struct Foo;
//! argtea_impl! {
//!     {}
//!     impl Foo {
//!         pub const FLAGS: &'static [argtea::Flag] = docs!();
//!         pub fn parse() {}
//!     }
//! }
//! ```
//! ## Constants
//! The right-hand-side of an [`argtea_impl`] constant can only be a macro
//! invokation. Additionally, the path to the macro cannot start with `::`.
//!
//! The following will not compile because the right-hand-side is not a macro
//! invokation:
//! ```compile_fail
//! # use argtea::argtea_impl;
//! # pub struct Foo;
//! argtea_impl! {
//!     {}
//!     impl Foo {
//!         const HELP: &'static str = "help message";
//!     }
//! }
//! ```
//!
//! ## Functions
//! `parse!()` invokations will not be processed if they are inside of a code
//! block. Additionally, every statement MUST end with a semicolon (even if
//! statements).
//!
//! The following will not compile because there is not a semicolon after the
//! if statement and the `parse!()` invocation is in an if statement.
//! ```compile_fail
//! # use argtea::argtea_impl;
//! # pub struct Foo;
//! argtea_impl! {
//!     {}
//!     impl Foo {
//!         fn parse(cond: bool) {
//!             let mut args = std::env::args().skip(1);
//!             if cond {
//!                 parse!(args);
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Formatting
//! There are two approaches to formatting with argtea:
//!   1. Runtime formatting
//!   2. Formatting macros
//!
//! With either approach, if an item is annotated with `#[hidden]`, it will not
//! be provided by `docs!()`.
//!
//! ## Runtime formatting
//! Runtime formatters are just regular functions that take in [`&[argtea::Flag]`](Flag).
//!
//! An [`&[argtea::Flag]`](Flag) slice can be obtained by adding a `docs!()` constant
//! to the [`argtea_impl`] macro invokation. ie:
//! ```rust
//! # use argtea::argtea_impl;
//! # pub struct Foo;
//! argtea_impl! {
//!     {/* ... */}
//!     impl Foo {
//!         const FLAGS: &'static [argtea::Flag] = docs!();
//!     }
//! }
//! ```
//! Note: all of such constants are `pub`. Writing `pub const` will result in a
//! compiler error.
//!
//! ## Formatting macros
//! Formatting macros are just regular macros that take in the following pattern:
//! ```text
//! [
//!     $({
//!         doc: [
//!             $( $doc:literal ),*
//!         ],
//!         flags: [
//!             $( $flag:literal ),*
//!         ],
//!         params: [
//!             $( $param:ident ),*
//!         ]
//!     }),*
//! ]
//! ```
//! When the following is written in the [`argtea_impl`] macro, the first
//! `docs!()` parameter is replaced with the above pattern. Then, the
//! [`simple_format`] macro is called:
//! ```rust
//! # use argtea::{simple_format, argtea_impl};
//! # pub struct Foo;
//! argtea_impl! {
//!     { /* ... */ }
//!     impl Foo {
//!         const HELP: &'static str = simple_format!("a" docs!() "b");
//!     }
//! }
//! ```

mod docs;
mod formatters;
mod help;

#[doc(hidden)]
pub mod parse;

pub use docs::Flag;
pub use help::wrapping_format;

#[macro_export]
macro_rules! argtea_impl {
    {
        $flags:tt
        impl $ty:ident {
            $(
                const $constant_name:ident: $constant_type:ty = $($macro:ident)::+ ! $mac_args:tt;
            )*
            $(
                fn $fn_name:ident $args:tt $(-> $ret_ty:ty)? {$($body:tt)*}
            )*
        }
    } => {

        impl $ty {
            $(
                pub const $constant_name: $constant_type = $crate::_filter_hidden_flags!($flags _constant_expression!($($macro)::+ ! $mac_args));
            )*
            $(
                pub fn $fn_name $args $(-> $ret_ty)? {
                    $crate::_scan_body!{
                        $flags $($body)*
                    }
                }
            )*
        }
    };
}
