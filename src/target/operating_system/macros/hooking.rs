#[macro_export]
macro_rules! hooking {
    ($identifier:ident) => {
        pub const NUMBER: usize = super::numbers::$identifier;
        pub const LABEL: &str = super::labels::$identifier;
        // pub type SIGNATURE = super::signatures::$identifier;
    };
}
pub use hooking;
