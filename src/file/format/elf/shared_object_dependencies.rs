//! Shared object dependencies described by the ELF dynamic array.

use ample::r#type::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedObjectDependency<'file> {
    pub dynamic_entry_index: usize,
    pub name: &'file str,
}

impl<'file> SharedObjectDependency<'file> {
    pub const fn new(dynamic_entry_index: usize, name: &'file str) -> Self {
        Self {
            dynamic_entry_index,
            name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPath<'file> {
    RunPath(&'file str),
    RuntimeSearchPath(&'file str),
}

#[derive(Debug)]
pub struct SharedObjectDependencies<'file> {
    pub needed: Vec<SharedObjectDependency<'file>>,
    pub shared_object_name: Option<&'file str>,
    pub runtime_search_path: Option<&'file str>,
    pub run_path: Option<&'file str>,
}

impl<'file> SharedObjectDependencies<'file> {
    pub const fn new(
        needed: Vec<SharedObjectDependency<'file>>,
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

    pub const fn search_path(&self) -> Option<SearchPath<'file>> {
        match (self.run_path, self.runtime_search_path) {
            (Some(run_path), _) => Some(SearchPath::RunPath(run_path)),
            (None, Some(runtime_search_path)) => {
                Some(SearchPath::RuntimeSearchPath(runtime_search_path))
            }
            (None, None) => None,
        }
    }
}
