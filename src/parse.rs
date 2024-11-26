pub struct FlagView {
    buf: [u8; 5],
}

impl FlagView {
    pub fn new() -> Self {
        Self { buf: [0; 5] }
    }

    pub fn get(&mut self, flag: char) -> &mut str {
        self.buf[0] = b'-';
        let len = flag.encode_utf8(&mut self.buf[1..]).len();

        unsafe { core::str::from_utf8_unchecked_mut(&mut self.buf[0..len + 1]) }
    }
}

/// Helper macro: removes all `#[fake]` flags and then calls the provided
/// macro with the filtered flags as the first argument.
#[doc(hidden)]
#[macro_export]
macro_rules! _filter_fake_flags {
    {
        $(@pre_flags {$($pre_flags:tt)*})?
        {}
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::$local_macro_to_call!({$($($pre_flags)*)?} $($other_args)*)
    };

    {
        $(@pre_flags {$($pre_flags:tt)*})?
        { $(#[doc = $cmnt1:literal])* #[fake] $(#[doc = $cmnt2:literal])* ($($lhs:tt)*) => $rhs:tt $($remaining:tt)* }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_fake_flags! {
            @pre_flags {$($($pre_flags)*)?}
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };

    {
        $(@pre_flags {$($pre_flags:tt)*})?
        { $(#[hidden])? $(#[doc = $cmnt:literal])* ($($lhs:tt)*) => $rhs:tt $($remaining:tt)* }
        $local_macro_to_call:ident!($($other_args:tt)*)
    } => {
        $crate::_filter_fake_flags! {
            @pre_flags {$($($pre_flags)*)? $(#[doc = $cmnt])* ($($lhs)*) => $rhs}
            {$($remaining)*}
            $local_macro_to_call!($($other_args)*)
        }
    };
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
                $(#[hidden])?
                ($($pat:tt)+) => $block:block
            )*
        }
    } => {
        #[allow(unused_variables)]
        {
            // For splitting flags like '-sw 80' => '-s -w 80'
            let mut flag_buf = String::new();
            let mut charview = $crate::parse::FlagView::new();

            // Stores the value in `--flag=value`
            let mut stashed_value = None;

            while let Some(mut flag) = if flag_buf.is_empty() {
                    $iter.next().map(::std::borrow::Cow::from)
                } else {
                    Some(::std::borrow::Cow::from(&*charview.get(flag_buf.remove(0))))
                }
            {
                if flag.starts_with("-") && !flag.starts_with("--") && flag.chars().count() > 2 {
                    flag_buf = flag.into_owned();
                    flag_buf.remove(0);
                    continue;
                }

                if flag.starts_with("--") {
                    if let Some(idx) = flag.find('=') {
                        let flag = flag.to_mut();
                        stashed_value = Some(flag.split_off(idx + 1));
                        flag.pop();
                    }
                }

                let mut $iter = ::core::iter::from_fn(|| (!flag_buf.is_empty()).then_some(::core::mem::take(&mut flag_buf)))
                    .chain(
                        stashed_value.take().into_iter().chain(&mut $iter)
                    );

                match &*flag {
                    $(
                        $crate::_create_branch_pat!(($($pat)+)) => $crate::_create_branch!($iter flag ($($pat)+) => $block),
                    )*
                }
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
        parse!($expr:expr);
        $($($rem:tt)+)?
    } => {
        let mut args = $expr;
        $crate::_parse!{
            args => $flags
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
        compile_error!("Invalid arguments to `parse!()` expected `parse!($expr)`")
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
        let $ident = $string.into_owned();
        $block
    }};
    {
        $iter:ident $string:ident ($($flag:literal)|+ $(, $param:ident)* $(,)? ) => $block:block
    } => {{
        $(let $param = $iter.next();)*
        $block
    }};
}
