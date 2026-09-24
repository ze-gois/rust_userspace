ample::result!(
    Ok;
    "Userspace success";
    usize;
    [
        [0; USERSPACE_TARGET_SUCCESS; Target; crate::target::Ok; "target"; "Target-domain success"]
    ];
    Error;
    "Userspace failure";
    usize;
    [
        [0; USERSPACE_TARGET_FAILURE; Target; crate::target::Error; "target"; "Target-domain failure"]
    ]
);

pub type Result = core::result::Result<Ok, Error>;
