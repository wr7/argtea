mod help;

pub use help::format_help;

#[macro_export]
macro_rules! impl_parse {
    {
        $flags:tt
        impl $ty:ident {
            $(
                const $flags_name:ident = docs!();
            )?
            $(
                fn $fn_name:ident $args:tt $(-> $ret_ty:ty)? {$($body:tt)*}
            )*
        }
    } => {

        impl $ty {
            $(
                pub const $flags_name: &'static [$crate::Flag] = $crate::_docs!($flags);
            )?
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

/// Helper macro; corresponds to `docs!()`.
///
/// The argument corresponds to the flags given to `impl_parse`
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
/// The `=> {...}` argument corresponds to the flags given to `impl_parse`
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
    {
        $iter:expr => {
            $(
                $(#[doc = $doc:literal])*
                ($($flag:literal)|* $(,)? $($param:ident),* $(,)? ) => $block:block
            ),* $(,)?
        }
    } => {
        compile_error!("The `parse` macro expects a mutable variable name and not an expression")
    }
}

/// Recursive helper macro. This replaces occurances of `parse!()` with
/// `$crate::_parse` and provides it the additional required arguments
///
/// The `flags` argument corresponds to the flags given to `impl_parse` (before
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
