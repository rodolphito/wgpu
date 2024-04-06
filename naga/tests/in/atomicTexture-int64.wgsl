@group(0) @binding(0)
var image: texture_2d<u64>;

@compute
@workgroup_size(2)
fn cs_main(@builtin(local_invocation_id) id: vec3<u32>) {
    atomicMax(&image, 1lu);

    workgroupBarrier();

    atomicMin(&image, 1lu);
}
