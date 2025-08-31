#[macro_export]
macro_rules! expressions_lowerbound {
    // caso base: apenas um argumento
    ($x:expr) => { $x };
    // caso recursivo: pelo menos dois
    ($x:expr, $($rest:expr),+) => {
        {
            let y = expressions_lowerbound!($($rest),+);
            if $x > y { y } else { $x }
        }
    };
}
pub use expressions_lowerbound;
