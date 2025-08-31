pub struct Stdout;

fn print(msg: &str) -> crate::Result {
    let out = crate::target::operating_system::syscall::write(1, msg.as_ptr(), msg.len());

    match out {
        _ => Ok(crate::Ok::Target(crate::target::Ok::Info(32))),
    }
}

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! info {
    (X $($arg:tt)*) => {{
        // use core::fmt::Write;
        // let mut writer = $crate::stdout::Stdout;
        // let _ = write!(&mut writer, $($arg)*);
    }};
    (S $($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::stdout::Stdout;
        let _ = write!(&mut writer, $($arg)*);
    }};
    ($($arg:tt)*) => {{

        // let file = file!();
        // let out = info::syscall::write(1, file.as_ptr(), file.len());

        use core::fmt::Write;
        let mut writer = crate::target::os::macros::info::Stdout;
        // let _ = write!(&mut writer, "\n[ {}:{}:{} ] ",file!(),line!(),column!());
        let _ = write!(&mut writer, $($arg)*);
    }};
}

#[macro_export]
macro_rules! tinfo {
    (X $($arg:tt)*) => {{
        // use core::fmt::Write;
        // let mut writer = $crate::stdout::Stdout;
        // let _ = write!(&mut writer, $($arg)*);
    }};
    (S $($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::stdout::Stdout;
        let _ = write!(&mut writer, $($arg)*);
    }};
    ($($arg:tt)*) => {{

        // let file = file!();
        // let out = info::syscall::write(1, file.as_ptr(), file.len());

        use core::fmt::Write;
        let mut writer = crate::target::os::macros::info::Stdout;
        // let _ = write!(&mut writer, "\n[ {}:{}:{} ] ",file!(),line!(),column!());
        let _ = write!(&mut writer, $($arg)*);
    }};
}
pub use info;
pub use tinfo;
