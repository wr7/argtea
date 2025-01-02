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
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {}
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::$local_macro_to_call!{{$($($pre_flags)*)?} $($other_args)*}
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {
            #[hidden]
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)*)?}
                attrs: {$($($attrs)*)?}
                hidden: true
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {
            #[fake]
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)*)?}
                attrs: {$($($attrs)*)?}
                hidden: $($($hidden)?)?
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {
            #[doc = $cmt:literal]
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)*)?}
                attrs: {$($($attrs)*)? #[doc = $cmt]}
                hidden: $($($hidden)?)?
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {
            #[$($attr:tt)*]
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        compile_error!(::core::concat!("Invalid flag attribute #[", ::core::stringify!($($attr)*), "]"))
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $($hidden:ident)?
        })?
        {
            ($flag_binding:ident @ $($lhs:tt)*) => $rhs:tt
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)* $($attrs)*)? ($($lhs)*) => $rhs}
                attrs: {}
                hidden:
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden:
        })?
        {
            ($($lhs:tt)*) => $rhs:tt
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)* $($attrs)*)? ($($lhs)*) => $rhs}
                attrs: {}
                hidden:
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@{
            pre_flags: {$($pre_flags:tt)*}
            attrs: {$($attrs:tt)*}
            hidden: $hidden:ident
        })?
        {
            ($($lhs:tt)*) => $rhs:tt
            $($remaining:tt)*
        }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_hidden_flags! {
            @{
                pre_flags: {$($($pre_flags)*)?}
                attrs: {}
                hidden:
            }
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };
}
