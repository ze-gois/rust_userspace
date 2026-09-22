//! Shared object dependencies described by the ELF dynamic array.

use ample::r#type::Vec;

#[derive(Debug)]
pub struct SharedObjectDependencies<'file> {
    pub needed: Vec<&'file str>,
    pub shared_object_name: Option<&'file str>,
    pub runtime_search_path: Option<&'file str>,
    pub run_path: Option<&'file str>,
}

impl<'file> SharedObjectDependencies<'file> {
    pub const fn new(
        needed: Vec<&'file str>,
        shared_object_name: Option<&'file str>,
        runtime_search_path: Option<&'file str>,
        run_path: Option<&'file str>,
    ) -> Self {
        Self {
            needed,
            shared_object_name,
            runtime_search_path,
            run_path,
        }
    }
}
