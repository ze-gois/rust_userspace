use super::super::super::dtype::class_32::*;

pub union DynamicUnion32 {
    pub d_val: Word,
    pub d_ptr: Addr,
}

ample::r#struct!(
    pub struct Dynamic32 {
        pub d_tag: SWord,
        pub d_un: DynamicUnion32,
    }
);

// extern pub _DYNAMIC[]: Dyn,
