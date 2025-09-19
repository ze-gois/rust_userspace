ample::result!(
    Ok;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_DEFAULT_OK; Default; usize; "ZE"; "Entry to ze"],
        [3; USERSPACE_MEMORY_HEAP_ALLOCATE_DEFAULT_OK; HeapAllocate; super::heap::AllocatorPointer; "ZE"; "Entry to ze"],
    ];
    Error;
    "Allocate Ok";
    usize;
    [
        [1; USERSPACE_MEMORY_DEFAULT_ERROR; Default; usize; "ZE"; "Entry to ze"],
        [3; USERSPACE_MEMORY_ALLOCATE_DEFAULT_ERROR; HeapAllocate; super::heap::AllocatorPointer; "ZE"; "Entry to ze"],
    ]
);
