#[macro_export]
macro_rules! trait_implement_partial_eq {
    ($name:ty, $inner:ty, $($t:ty),*) => {
        $(
            impl core::cmp::PartialEq<$t> for $name {
                fn eq(&self, other: &$t) -> bool {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        self.0 == *other as $inner
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialEq<$name> for $t {
                fn eq(&self, other: &$name) -> bool {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        *self as $inner == other.0
                    } else {
                        false
                    }
                }
            }

            impl core::cmp::PartialOrd<$t> for $name {
                fn partial_cmp(&self, other: &$t) -> Option<core::cmp::Ordering> {
                    if (*other as u128) <= <$inner>::MAX as u128 {
                        Some(self.0.cmp(&(*other as $inner)))
                    } else {
                        None
                    }
                }
            }

            impl core::cmp::PartialOrd<$name> for $t {
                fn partial_cmp(&self, other: &$name) -> Option<core::cmp::Ordering> {
                    if (*self as u128) <= <$inner>::MAX as u128 {
                        Some((*self as $inner).cmp(&other.0))
                    } else {
                        None
                    }
                }
            }
        )*
    }
}
pub use trait_implement_partial_eq;
