@group(0) @binding(0)
<<<<<<< HEAD
var image: texture_storage_2d<r64uint, write>;
=======
var image: texture_2d<u64>;
>>>>>>> 8a58c93978816df03073f4223379775ee96476ba

@compute
@workgroup_size(2)
fn cs_main(@builtin(local_invocation_id) id: vec3<u32>) {
<<<<<<< HEAD
    imageAtomicMax(image, vec2<i32>(0, 0), 1lu);

    workgroupBarrier();

    imageAtomicMin(image, vec2<i32>(0, 0), 1lu);
=======
    atomicMax(&image, 1lu);

    workgroupBarrier();

    atomicMin(&image, 1lu);
>>>>>>> 8a58c93978816df03073f4223379775ee96476ba
}
