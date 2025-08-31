use crate::target::os::syscall;

pub fn print(filepath: &str) {
    let (_f, s, p) = crate::file::load(filepath).unwrap();
    let _ = syscall::write(1, p, s.st_size as usize);
}
