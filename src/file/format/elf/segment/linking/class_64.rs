use super::super::super::dtype::class_64::*;

pub union DynamicUnion64 {
    pub d_val: XWord,
    pub d_ptr: Addr,
}

ample::r#struct!(
    pub struct Dynamic64 {
        pub d_tag: SXWord,
        pub d_un: DynamicUnion64,
    }
);
