#[macro_export]
macro_rules! expressions_upperbound {
    // caso base: apenas um argumento
    ($x:expr) => { $x };
    // caso recursivo: pelo menos dois
    ($x:expr, $($rest:expr),+) => {
        {
            let y = expressions_upperbound!($($rest),+);
            if $x > y { $x } else { y }
        }
    };
}
pub use expressions_upperbound;
