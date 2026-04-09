macro_rules! collects_enum {
    ($($root:ident::$variant:ident => $base:ident),* $(,)?) => {
        $(
            impl From<$base> for $root {
                fn from(given_base: $base) -> Self {
                    $root::$variant(given_base)
                }
            }
        )*
    };
}

pub(crate) use collects_enum;
