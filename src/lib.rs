//! Declarative macro commandline parser (inspired by argwerk).
//!
//! argtea attempts to reduce abstraction and maximize flexibility.
//!
//! ## Comparison to argwerk
//!
//! |                                      | `argtea`  | `argwerk` |
//! | :----------------------------------- | :-------: | :-------: |
//! | `--flag=value` syntax                | ✓         | ✗         |
//! | `-sw 80` <=> `-s -w 80` syntax       | ✓         | ✗         |
//! | `-Wall`  <=> `-W all` syntax         | ✓         | ✗         |
//! | OsString argument support            | ✗         | ✓         |
//! | Customizable help message formatting | ✓         | ✓*        |
//! | Help message generation              | ✓         | ✓*        |
//!
//! \[*\] At runtime
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
//! ## Functions
//! Argtea functions are defined with syntax similar to regular rust functions. Note: unlike
//! rust functions, visibility and generics cannot be specified. All argtea functions are `pub`
//! regardless of how they're declared.
//!
//! Argtea functions can use the `parse!()` macro which takes in a `String` iterator. It will then
//! parse it using the flags and code defined above.
//!
//! However, argtea functions have the following limitations:
//! 1. ALL statements must be terminated by semicolons (even if statements and loops).
//!     a. If you don't do this, you will get a cryptic compiler error.
//! 2. `parse!()` invokations cannot be inside of code blocks (such as if statements).
//!
//! ## Constants
//! There are two types of argtea constants:
//! 1. Flag constants:
//!   ```rust
//!   # use argtea::argtea_impl;
//!   # pub struct Foo;
//!   argtea_impl! {
//!       {/* ... */}
//!       impl Foo {
//!           const FLAGS: &'static [argtea::Flag] = docs!();
//!       }
//!   }
//!   ```
//! 2. Macro constants:
//!   ```rust
//!   # use argtea::{simple_format, argtea_impl};
//!   # pub struct Foo;
//!   argtea_impl! {
//!       { /* ... */ }
//!       impl Foo {
//!           const HELP: &'static str = simple_format!("a" docs!() "b");
//!       }
//!   }
//!   ```
//!
//! The first type of constant generates an [`Flag`] for each non-`#[hidden]` flag. This can
//! be used to generate help messages and other information at run-time.
//!
//! Macro constants call macros with information about the non-`#[hidden]` flags. These can be used
//! for compile-time help message generation. This crate provides the [`simple_format`] macro which
//! provides simple, compile-time help message generation. For more information about formatting
//! macros, see the "Formatting macros" section below.
//!
//! ## `#[hidden]` and `#[fake]`
//!
//! Flags can optionally be annotated with `#[hidden]` or `#[fake]`. `#[hidden]` hides a flag from
//! the documentation while `#[fake]` shows a flag in the documentation that doesn't really exist.
//!
//! The following is an example where `#[fake]` and `#[hidden]` come in handy:
//! ```rust
//! # use argtea::{argtea_impl, Flag};
//! # struct Foo;
//! # argtea_impl! {{
//! /// Enables all warnings
//! #[fake]
//! ("-Wall") => {}
//!
//! // In this example, argtea interprets `-Wall` as `-W all`, so it will be matched to this flag.
//! // This would be the case even if the above flag wasn't annotated with `#[fake]`
//! //
//! // However, the user may still wish to display `-Wall` as a separate flag in their documentation.
//! ("-W" | "--warning", warning) => { /* ... */ }
//!
//! // This flag is just here for error handling (when the user passes in an invalid flag)
//! // Therefore, this shouldn't be shown in the documentation
//! #[hidden]
//! (invalid_flag) => { /* ... */ }
//! # }
//! # impl Foo {
//! # const a: &[Flag] = docs!();
//! # fn foo() {parse!(None.into_iter());}
//! # }
//! # }
//! ```
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

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! argtea_impl {
    {
        $flags:tt
        impl $ty:ident {
            $(
                $(pub)? const $constant_name:ident: $constant_type:ty = $($macro:ident)::+ ! $mac_args:tt;
            )*
            $(
                $(pub)? fn $fn_name:ident $args:tt $(-> $ret_ty:ty)? {$($body:tt)*}
            )*
        }
    } => {

        impl $ty {
            $(
                pub const $constant_name: $constant_type = $crate::_filter_hidden_flags!($flags _constant_expression!($($macro)::+ ! $mac_args));
            )*
            $(
                pub fn $fn_name $args $(-> $ret_ty)? {
                    $crate::_filter_fake_flags!{
                        $flags
                        _scan_body!(
                            $($body)*
                        )
                    }
                }
            )*
        }
    };

    {
        $flags:tt
        impl $ty:ident {
            $(
                $(pub)? fn $fn_name:ident $args:tt $(-> $ret_ty:ty)? {$($body:tt)*}
            )*
            $(
                $(pub)? const $constant_name:ident: $constant_type:ty = $($macro:ident)::+ ! $mac_args:tt;
            )+
            $(
                $(pub)? fn $fn_name2:ident $args2:tt $(-> $ret_ty2:ty)? {$($body2:tt)*}
            )*
        }
    } => {compile_error!("Argtea constants must be defined before functions")};

    {
        $flags:tt
        impl $ty:ident {
            $(
                $(pub)? const $constant_name:ident: $constant_type:ty = $($macro:ident)::+ ! $mac_args:tt;
            )*
            $(
                $(pub)? fn $fn_name:ident $args:tt $(-> $ret_ty:ty)? {$($body:tt)*}
            )+
            $(
                $(pub)? const $constant_name2:ident: $constant_type2:ty = $($macro2:ident)::+ ! $mac_args2:tt;
            )*
        }
    } => {compile_error!("Argtea constants must be defined before functions")};
}
