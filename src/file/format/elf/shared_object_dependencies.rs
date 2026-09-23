//! Shared object dependencies described by the ELF dynamic array.

use ample::r#type::{String, Vec};

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

    pub fn name_with_origin(
        &self,
        origin_directory: &str,
    ) -> Result<String, OriginSubstitutionError> {
        substitute_origin(self.name, origin_directory)
    }

    pub fn direct_pathname(
        &self,
        origin_directory: &str,
    ) -> Result<Option<String>, OriginSubstitutionError> {
        let name = self.name_with_origin(origin_directory)?;
        Ok(name.contains('/').then_some(name))
    }

    pub fn requires_search(
        &self,
        origin_directory: &str,
    ) -> Result<bool, OriginSubstitutionError> {
        Ok(self.direct_pathname(origin_directory)?.is_none())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OriginSubstitutionError {
    UnspecifiedSequence { byte_offset: usize },
}

fn is_name_start(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphabetic()
}

fn is_name_continue(byte: u8) -> bool {
    is_name_start(byte) || byte.is_ascii_digit()
}

fn substitute_origin(
    value: &str,
    origin_directory: &str,
) -> Result<String, OriginSubstitutionError> {
    let bytes = value.as_bytes();
    let mut result = String::with_capacity(value.len());
    let mut literal_start = 0usize;
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] != b'    Pathname(&'file str),
    CurrentDirectory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPath<'file> {
    RunPath(&'file str),
    RuntimeSearchPath(&'file str),
}

impl<'file> SearchPath<'file> {
    pub const fn value(self) -> &'file str {
        match self {
            Self::RunPath(value) | Self::RuntimeSearchPath(value) => value,
        }
    }

    pub fn value_with_origin(
        self,
        origin_directory: &str,
    ) -> Result<String, OriginSubstitutionError> {
        match self {
            Self::RunPath(value) => substitute_origin(value, origin_directory),
            Self::RuntimeSearchPath(value) => Ok(String::from(value)),
        }
    }

    pub fn directories(self) -> Vec<SearchDirectory<'file>> {
        let mut directories = Vec::new();

        for directory in self.value().split(':') {
            if directory.is_empty() {
                directories.push(SearchDirectory::CurrentDirectory);
            } else {
                directories.push(SearchDirectory::Pathname(directory));
            }
        }

        directories
    }
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
 {
            index += 1;
            continue;
        }

        result.push_str(&value[literal_start..index]);
        let sequence_start = index;
        index += 1;

        let name = if bytes.get(index) == Some(&b'{') {
            let name_start = index + 1;
            let Some(relative_end) = bytes[name_start..].iter().position(|byte| *byte == b'}') else {
                return Err(OriginSubstitutionError::UnspecifiedSequence {
                    byte_offset: sequence_start,
                });
            };
            let name_end = name_start + relative_end;
            let name = &value[name_start..name_end];

            let mut name_bytes = name.as_bytes().iter().copied();
            if !name_bytes.next().is_some_and(is_name_start)
                || !name_bytes.all(is_name_continue)
            {
                return Err(OriginSubstitutionError::UnspecifiedSequence {
                    byte_offset: sequence_start,
                });
            }

            index = name_end + 1;
            name
        } else {
            let Some(first) = bytes.get(index).copied() else {
                return Err(OriginSubstitutionError::UnspecifiedSequence {
                    byte_offset: sequence_start,
                });
            };
            if !is_name_start(first) {
                return Err(OriginSubstitutionError::UnspecifiedSequence {
                    byte_offset: sequence_start,
                });
            }

            let name_start = index;
            index += 1;
            while bytes.get(index).copied().is_some_and(is_name_continue) {
                index += 1;
            }
            &value[name_start..index]
        };

        if name != "ORIGIN" {
            return Err(OriginSubstitutionError::UnspecifiedSequence {
                byte_offset: sequence_start,
            });
        }

        result.push_str(origin_directory);
        literal_start = index;
    }

    result.push_str(&value[literal_start..]);
    Ok(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirectory<'file> {
    Pathname(&'file str),
    CurrentDirectory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchPath<'file> {
    RunPath(&'file str),
    RuntimeSearchPath(&'file str),
}

impl<'file> SearchPath<'file> {
    pub const fn value(self) -> &'file str {
        match self {
            Self::RunPath(value) | Self::RuntimeSearchPath(value) => value,
        }
    }

    pub fn directories(self) -> Vec<SearchDirectory<'file>> {
        let mut directories = Vec::new();

        for directory in self.value().split(':') {
            if directory.is_empty() {
                directories.push(SearchDirectory::CurrentDirectory);
            } else {
                directories.push(SearchDirectory::Pathname(directory));
            }
        }

        directories
    }
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
