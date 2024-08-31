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
            let mut flag_buf = String::new();
            let mut charview = $crate::parse::FlagView::new();

            while let Some(flag) = if flag_buf.is_empty() {
                    $iter.next().map(|s| ::std::borrow::Cow::from(s))
                } else {
                    Some(::std::borrow::Cow::from(&*charview.get(flag_buf.remove(0))))
                }
            {

                if flag.starts_with("-") && !flag.starts_with("--") && flag.chars().count() > 2 {
                    flag_buf = flag.into_owned();
                    flag_buf.remove(0);
                    continue;
                }

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
