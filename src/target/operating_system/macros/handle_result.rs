macro_rules! handle_result {
    ($($body:tt)*) => {
        fn handle_result(result: crate::Result) -> crate::Result {
            match result {
                Ok(signed_result) => Ok(signed_result),
                Err(err) => {
                    let errno : isize = err.into();

                    human::info!("\nRaw syscall error: {} = {}\n",
                                SYSCALL_NAME, errno);

                    $($body)*

                    Err(crate::Error::$result_variant(matched_error))
                }
            }
        }
    };
}


fn handle_result(result: crate::target::Result<isize>) -> crate::result::Result<isize> {
    match result {
        Ok(signed_result) => Ok(signed_result),
        Err(err) => {
            let errno : isize = err.into();

            human::info!("\nRaw syscall error: {} = {}\n",
                        SYSCALL_NAME, errno);

            // let abs_errno = if errno < 0 { -errno } else { errno };

            // Create appropriate error based on the error code
            let matched_error = match errno {
                $($errno => $error_enum_name::$error_variant,)*
                _ => $error_enum_name::TODO
            };

            Err(crate::result::Error::$result_variant(matched_error))
        }
    }
}
pub use handle_result;
