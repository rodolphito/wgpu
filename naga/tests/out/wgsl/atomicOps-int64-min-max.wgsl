struct Struct {
    atomic_scalar: atomic<u64>,
    atomic_arr: array<atomic<u64>, 2>,
}

@group(0) @binding(0) 
var<storage, read_write> storage_atomic_scalar: atomic<u64>;
@group(0) @binding(1) 
var<storage, read_write> storage_atomic_arr: array<atomic<u64>, 2>;
@group(0) @binding(2) 
var<storage, read_write> storage_struct: Struct;
var<workgroup> workgroup_atomic_scalar: atomic<u64>;
var<workgroup> workgroup_atomic_arr: array<atomic<u64>, 2>;
var<workgroup> workgroup_struct: Struct;

@compute @workgroup_size(2, 1, 1) 
fn cs_main(@builtin(local_invocation_id) id: vec3<u32>) {
    let _e3 = atomicMax((&storage_atomic_scalar), 1lu);
    let _e7 = atomicMax((&storage_atomic_arr[1]), 1lu);
    let _e11 = atomicMax((&storage_struct.atomic_scalar), 1lu);
    let _e16 = atomicMax((&storage_struct.atomic_arr[1]), 1lu);
    let _e19 = atomicMax((&workgroup_atomic_scalar), 1lu);
    let _e23 = atomicMax((&workgroup_atomic_arr[1]), 1lu);
    let _e27 = atomicMax((&workgroup_struct.atomic_scalar), 1lu);
    let _e32 = atomicMax((&workgroup_struct.atomic_arr[1]), 1lu);
    workgroupBarrier();
    let _e35 = atomicMin((&storage_atomic_scalar), 1lu);
    let _e39 = atomicMin((&storage_atomic_arr[1]), 1lu);
    let _e43 = atomicMin((&storage_struct.atomic_scalar), 1lu);
    let _e48 = atomicMin((&storage_struct.atomic_arr[1]), 1lu);
    let _e51 = atomicMin((&workgroup_atomic_scalar), 1lu);
    let _e55 = atomicMin((&workgroup_atomic_arr[1]), 1lu);
    let _e59 = atomicMin((&workgroup_struct.atomic_scalar), 1lu);
    let _e64 = atomicMin((&workgroup_struct.atomic_arr[1]), 1lu);
    return;
}
