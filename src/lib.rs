//! Declarative macro commandline parser (inspired by argwerk).
//!
//! The goal of argtea (pronounced arg tea) is to automatically generate help
//! pages using doc comments. argtea attempts to be more flexible and less
//! abstracted than argwerk.
//!
//! Example project:
//! ```rust
//! use argtea::{argtea_impl, simple_format};
//!
//! #[derive(Debug)]
//! pub struct Arguments {
//!     output_path: String,
//!     files: Vec<String>,
//! }
//!
//! fn main() -> Result<(), &'static str> {
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
//!         },
//!
//!         /// Sets the output file path.
//!         ("--output" | "-o", output_path) => {
//!             output_path_ = output_path;
//!         },
//!
//!         /// Adds a file as an input.
//!         ///
//!         /// To input a file that starts with a `-`, prefix it with a `./`
//!         (file) => {
//!             files.push(file);
//!         },
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
//!         fn parse() -> Result<Self, &'static str> {
//!             let mut args = std::env::args().skip(1);
//!
//!             let mut files = Vec::new();
//!             let mut output_path_ = None;
//!
//!             parse!(args);
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
//! output from `argtea_test -h`:
//! ```text
//! argtea_test: a demo argtea project
//!
//! Usage:
//!   `argtea_test [FLAGS] [FILES]`
//!
//! Options:
//!   --help, -h
//!     Displays this help message.
//!
//!
//!   --output, -o <output_path>
//!     Sets the output file path.
//!
//!
//!   <file>
//!     Adds a file as an input.
//!
//!     To input a file that starts with a `-`, prefix it with a `./`
//!
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

mod formatters;
mod help;

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
                pub const $constant_name: $constant_type = $crate::_constant_expression!($flags $($macro)::+ ! $mac_args);
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

pub struct Flag {
    pub doc: &'static [&'static str],
    pub flags: &'static [&'static str],
    pub params: &'static [&'static str],
}

/// Helper macro; parses the right-hand-side of `const` items
///
/// The `flags` argument corresponds to the flags given to [`argtea_impl`]
/// (before the `impl`)
#[doc(hidden)]
#[macro_export]
macro_rules! _constant_expression {
    {
        $flags:tt
        docs!()
    } => {
        $crate::_docs!($flags)
    };

    {
        $flags:tt
        docs! $args:tt
    } => {
        ::core::compiler_error!("doc! macro does not accept arguments")
    };

    {
        {
            $(
                $(#[doc = $doc:literal])*
                ($($flag:literal)|* $(,)? $($param:ident),* $(,)? ) => $block:block
            ),* $(,)?
        }
        $(@ pre_args: {$($pre_args:tt)+})?
        $($macro:ident)::+ ! (docs!() $($post_args:tt)*)
    } => {
        $($macro)::+ !(
            $($($pre_args)+)?
            [
                $(
                    {
                        doc: [
                            $($doc),*
                        ],
                        flags: [
                            $($flag),*
                        ],
                        params: [
                            $($param),*
                        ]
                    }
                ),*
            ]
            $($post_args)*
        )
    };
    {
        $flags:tt
        $(@ pre_args: {$($pre_args:tt)+})?
        $($macro:ident)::+ ! ($pre_arg:tt $($rem:tt)+)
    } => {
        $crate::_constant_expression!(
            $flags @ pre_args: {$($($pre_args)+)? $pre_arg}
            $($macro)::+ ! ($($rem)+)
        )
    };

    {
        $flags:tt
        $(@ pre_args: {$($pre_args:tt)+})?
        $($macro:ident)::+ ! ($($args:tt)*)
    } => {
        $($macro:ident)::+ ! ($($args)*)
    };
}

/// Helper macro; corresponds to `docs!()`.
///
/// The argument corresponds to the flags given to [`argtea_impl`]
/// (before the `impl`)
#[doc(hidden)]
#[macro_export]
macro_rules! _docs {
    {
        {
            $(
                $(#[doc = $doc:literal])*
                ($($flag:literal)|* $(,)? $($param:ident),* $(,)? ) => $block:block
            ),* $(,)?
        }
    } => {
        &[
            $(
                $crate::Flag {
                    doc: &[
                        $($doc,)*
                    ],
                    flags: &[
                        $($flag,)*
                    ],
                    params: &[
                        $(::core::stringify!($param),)*
                    ],
                },
            )*
        ]
    }
}

/// Helper macro; corresponds to `parse!(iter)`.
///
/// The `=> {...}` argument corresponds to the flags given to [`argtea_impl`]
/// (before the `impl`)
#[doc(hidden)]
#[macro_export]
macro_rules! _parse {
    {
        $iter:ident => {
            $(
                $(#[doc = $doc:literal])*
                ($($pat:tt)+) => $block:block
            ),* $(,)?
        }
    } => {
        #[allow(unused_variables)]
        while let Some(flag) = $iter.next() {
            match &*flag {
                $(
                    $crate::_create_branch_pat!(($($pat)+)) => $crate::_create_branch!($iter flag ($($pat)+) => $block),
                )*
            }
        }
    };
}

/// Recursive helper macro. This replaces occurances of `parse!()` with
/// `$crate::_parse` and provides it the additional required arguments
///
/// The `flags` argument corresponds to the flags given to [`argtea_impl`] (before
/// the `impl`)
#[doc(hidden)]
#[macro_export]
macro_rules! _scan_body {
    {
        $flags:tt
        parse!($iter:ident);
        $($($rem:tt)+)?
    } => {
        $crate::_parse!{
            $iter => $flags
        }
        $(
            $crate::_scan_body!{
                $flags
                $($rem)+
            };
        )?
    };
    {
        $flags:tt
        parse! $args:tt;
        $($($rem:tt)+)?
    } => {
        compile_error!("Invalid arguments to `parse!()` expected `parse!($identifier)`")
    };
    {
        $flags:tt
        $expr:stmt;
        $($($rem:tt)+)?
    } => {
        $expr
        $(
            $crate::_scan_body!{
                $flags
                $($rem)+
            };
        )?
    };
    {$flags:tt} => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _create_branch_pat {
    {
        ($ident:ident)
    } => {
        _
    };
    {
        ($($flag:literal)|+ $(, $param:ident)* $(,)? )
    } => {
        $($flag)|+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _create_branch{
    {
        $iter:ident $string:ident ($ident:ident) => $block:block
    } => {{
        let $ident = $string;
        $block
    }};
    {
        $iter:ident $string:ident ($($flag:literal)|+ $(, $param:ident)* $(,)? ) => $block:block
    } => {{
        $(let $param = $iter.next();)*
        $block
    }};
}
