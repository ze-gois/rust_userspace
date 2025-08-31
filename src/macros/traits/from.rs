#[macro_export]
macro_rules! trait_implement_from {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl From<$name> for $t {
                fn from(val: $name) -> $t {
                    val.0 as $t
                }
            }

            impl From<$t> for $name {
                fn from(v: $t) -> Self {
                    Self(v as $inner)
                }
            }
        )*
    }
}
pub use trait_implement_from;
