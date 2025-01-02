pub struct Flag {
    pub doc: &'static [&'static str],
    pub flags: &'static [&'static str],
    pub params: &'static [&'static str],
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
            )*
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
            )*
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
        $($macro)::+ ! ($($args)*)
    };
}

/// Helper macro: removes all `#[hidden]` flags and then calls the provided
/// macro with the filtered flags as the first argument.
#[doc(hidden)]
#[macro_export]
macro_rules! _filter_hidden_flags {
    {
        $(@pre_flags {$($pre_flags:tt)*})?
        {}
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::$local_macro_to_call!{{$($($pre_flags)*)?} $($other_args)*}
    };

    {
        $(@pre_flags {$($pre_flags:tt)*})?
        { #[hidden] ($($lhs:tt)*) => $rhs:tt $($remaining:tt)* }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @pre_flags {$($($pre_flags)*)?}
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@pre_flags {$($pre_flags:tt)*})?
        { $(#[fake])? $(#[doc = $cmnt:literal])* $(#[fake])? ($($lhs:tt)*) => $rhs:tt $($remaining:tt)* }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @pre_flags {$($($pre_flags)*)? $(#[doc = $cmnt])* ($($lhs)*) => $rhs}
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };
}

/// Helper macro: removes all 'flag_name @' from the lhs of all flag declarations and then calls the
/// provided macro with the filtered flags as the first argument.
#[doc(hidden)]
#[macro_export]
macro_rules! _remove_flag_bindings {
    {
        {}
        $(@pre_flags {$($pre_flags:tt)*})?
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::$local_macro_to_call!{{$($($pre_flags)*)?} $($other_args)*}
    };

    {
        { #[hidden] ($($lhs:tt)*) => $rhs:tt $($remaining:tt)* }
        $(@pre_flags {$($pre_flags:tt)*})?
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_remove_flag_bindings! {
            {$($remaining)*}
            @pre_flags {$($($pre_flags)*)?}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        {
            $(#[doc = $cmnt:literal])*
            (
                $variable_name:ident @ $($lhs:tt)*
            ) => $rhs:tt
            $($remaining:tt)*
        }
        $(@pre_flags {$($pre_flags:tt)*})?
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_remove_flag_bindings! {
            {$($remaining)*}
            @pre_flags {$($($pre_flags)*)? $(#[doc = $cmnt])* ($($lhs)*) => $rhs}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        {
            $(#[doc = $cmnt:literal])*
            (
                $($lhs:tt)*
            ) => $rhs:tt $($remaining:tt)*
        }
        $(@pre_flags {$($pre_flags:tt)*})?
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_remove_flag_bindings! {
            {$($remaining)*}
            @pre_flags {$($($pre_flags)*)? $(#[doc = $cmnt])* ($($lhs)*) => $rhs}
            $local_macro_to_call!($($other_args)*)
        }
    };
}
