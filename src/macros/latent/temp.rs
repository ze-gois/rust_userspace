#[macro_export]
macro_rules! chain_latter {
    ($a:ident) => {
        $a
    };
    ($($t:tt);* $latter:tt) => {
        chain_latter!($latter)
    };
}
pub use chain_latter;
