#[unsafe(no_mangle)]
pub extern "C" fn flag_license() {
    crate::info!("It is goooood fo you\n");
    crate::file::print("LICENSE");
}
