#[macro_export]
macro_rules! trait_place_result {
    () => {
        pub trait Result {
            type ResultType;

            // fn from_no(no: Self::ResultType) -> Self;

            fn acronym(&self) -> &'static str;

            fn description(&self) -> &'static str;
        }
    };
}
pub use trait_place_result;
