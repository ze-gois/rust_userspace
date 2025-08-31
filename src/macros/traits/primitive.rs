#[macro_export]
macro_rules! trait_implement_primitive {
    ($tv:expr, $($t:ty),*) => {
        $(
            impl $crate::traits::Primitive<crate::Origin,crate::Origin> for $t {
                const IS_PRIMITIVE: bool = $tv;
            }
        )*
    };
}
pub use trait_implement_primitive;
