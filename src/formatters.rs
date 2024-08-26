/// Simple compile-time formatting of commandline options.
///
/// See crate-level documentation for usage
///
/// This macro automatically indents but does not automatically perform line
/// wrapping.
#[macro_export]
macro_rules! simple_format {
    {
        $($prefix:literal)*
        [
            $({
                doc: [
                    $( $doc:literal ),*
                ],
                flags: [
                    $(
                        $first_flag:literal,
                        $( $flag:literal ),*
                    )?
                ],
                params: [
                    $( $param:ident ),*
                ]
            }),*
        ]
        $($suffix:literal)*
    } => {
        ::core::concat!(
            $($prefix, "\n",)*
            $(
                "  ",
                $( $first_flag, $( ", ", $flag, )* " ", )?
                $( "<", ::core::stringify!($param), "> ", )*
                $("\n   ", $doc,)*
                "\n\n\n",
            )*
            $($suffix, "\n",)*
        )
    };
}
