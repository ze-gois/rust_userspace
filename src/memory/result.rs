ample::result!(
    Ok;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_MEMORY_ALLOCATE_OK; Allocate; crate::memory::allocation::Ok; "ZE"; "Entry to ze"],

    ];
    Error;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        [2; USERSPACE_MEMORY_ALLOCATE_ERROR; Allocate; crate::memory::allocation::Ok; "ZE"; "Entry to ze"],
    ]
);
