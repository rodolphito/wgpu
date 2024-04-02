struct NagaConstants {
    int first_vertex;
    int first_instance;
    uint other;
};
ConstantBuffer<NagaConstants> _NagaConstants: register(b0, space1);

struct Struct {
    uint64_t atomic_scalar;
    uint64_t atomic_arr[2];
};

RWByteAddressBuffer storage_atomic_scalar : register(u0);
RWByteAddressBuffer storage_atomic_arr : register(u1);
RWByteAddressBuffer storage_struct : register(u2);
groupshared uint64_t workgroup_atomic_scalar;
groupshared uint64_t workgroup_atomic_arr[2];
groupshared Struct workgroup_struct;

[numthreads(2, 1, 1)]
void cs_main(uint3 id : SV_GroupThreadID, uint3 __local_invocation_id : SV_GroupThreadID)
{
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        workgroup_atomic_scalar = (uint64_t)0;
        workgroup_atomic_arr = (uint64_t[2])0;
        workgroup_struct = (Struct)0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint64_t _e3; storage_atomic_scalar.InterlockedMax(0, 1uL, _e3);
    uint64_t _e7; storage_atomic_arr.InterlockedMax(8, 1uL, _e7);
    uint64_t _e11; storage_struct.InterlockedMax(0, 1uL, _e11);
    uint64_t _e16; storage_struct.InterlockedMax(8+8, 1uL, _e16);
    uint64_t _e19; InterlockedMax(workgroup_atomic_scalar, 1uL, _e19);
    uint64_t _e23; InterlockedMax(workgroup_atomic_arr[1], 1uL, _e23);
    uint64_t _e27; InterlockedMax(workgroup_struct.atomic_scalar, 1uL, _e27);
    uint64_t _e32; InterlockedMax(workgroup_struct.atomic_arr[1], 1uL, _e32);
    GroupMemoryBarrierWithGroupSync();
    uint64_t _e35; storage_atomic_scalar.InterlockedMin(0, 1uL, _e35);
    uint64_t _e39; storage_atomic_arr.InterlockedMin(8, 1uL, _e39);
    uint64_t _e43; storage_struct.InterlockedMin(0, 1uL, _e43);
    uint64_t _e48; storage_struct.InterlockedMin(8+8, 1uL, _e48);
    uint64_t _e51; InterlockedMin(workgroup_atomic_scalar, 1uL, _e51);
    uint64_t _e55; InterlockedMin(workgroup_atomic_arr[1], 1uL, _e55);
    uint64_t _e59; InterlockedMin(workgroup_struct.atomic_scalar, 1uL, _e59);
    uint64_t _e64; InterlockedMin(workgroup_struct.atomic_arr[1], 1uL, _e64);
    return;
}
