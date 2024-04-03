#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

struct Struct {
    uint atomic_scalar;
    int atomic_arr[2];
};
layout(std430) buffer type_block_0Compute { uint _group_0_binding_0_cs; };

layout(std430) buffer type_2_block_1Compute { int _group_0_binding_1_cs[2]; };

layout(std430) buffer Struct_block_2Compute { Struct _group_0_binding_2_cs; };

shared uint workgroup_atomic_scalar;

shared int workgroup_atomic_arr[2];

shared Struct workgroup_struct;


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        workgroup_atomic_scalar = 0u;
        workgroup_atomic_arr = int[2](0, 0);
        workgroup_struct = Struct(0u, int[2](0, 0));
    }
    memoryBarrierShared();
    barrier();
    uvec3 id = gl_LocalInvocationID;
    _group_0_binding_0_cs = 1u;
    _group_0_binding_1_cs[1] = 1;
    _group_0_binding_2_cs.atomic_scalar = 1u;
    _group_0_binding_2_cs.atomic_arr[1] = 1;
    workgroup_atomic_scalar = 1u;
    workgroup_atomic_arr[1] = 1;
    workgroup_struct.atomic_scalar = 1u;
    workgroup_struct.atomic_arr[1] = 1;
    memoryBarrierShared();
    barrier();
    uint l0_ = _group_0_binding_0_cs;
    int l1_ = _group_0_binding_1_cs[1];
    uint l2_ = _group_0_binding_2_cs.atomic_scalar;
    int l3_ = _group_0_binding_2_cs.atomic_arr[1];
    uint l4_ = workgroup_atomic_scalar;
    int l5_ = workgroup_atomic_arr[1];
    uint l6_ = workgroup_struct.atomic_scalar;
    int l7_ = workgroup_struct.atomic_arr[1];
    memoryBarrierShared();
    barrier();
    uint _e51 = atomicAdd(_group_0_binding_0_cs, 1u);
    int _e55 = atomicAdd(_group_0_binding_1_cs[1], 1);
    uint _e59 = atomicAdd(_group_0_binding_2_cs.atomic_scalar, 1u);
    int _e64 = atomicAdd(_group_0_binding_2_cs.atomic_arr[1], 1);
    uint _e67 = atomicAdd(workgroup_atomic_scalar, 1u);
    int _e71 = atomicAdd(workgroup_atomic_arr[1], 1);
    uint _e75 = atomicAdd(workgroup_struct.atomic_scalar, 1u);
    int _e80 = atomicAdd(workgroup_struct.atomic_arr[1], 1);
    memoryBarrierShared();
    barrier();
    uint _e83 = atomicAdd(_group_0_binding_0_cs, -1u);
    int _e87 = atomicAdd(_group_0_binding_1_cs[1], -1);
    uint _e91 = atomicAdd(_group_0_binding_2_cs.atomic_scalar, -1u);
    int _e96 = atomicAdd(_group_0_binding_2_cs.atomic_arr[1], -1);
    uint _e99 = atomicAdd(workgroup_atomic_scalar, -1u);
    int _e103 = atomicAdd(workgroup_atomic_arr[1], -1);
    uint _e107 = atomicAdd(workgroup_struct.atomic_scalar, -1u);
    int _e112 = atomicAdd(workgroup_struct.atomic_arr[1], -1);
    memoryBarrierShared();
    barrier();
    atomicMax(_group_0_binding_0_cs, 1u);
    atomicMax(_group_0_binding_1_cs[1], 1);
    atomicMax(_group_0_binding_2_cs.atomic_scalar, 1u);
    atomicMax(_group_0_binding_2_cs.atomic_arr[1], 1);
    atomicMax(workgroup_atomic_scalar, 1u);
    atomicMax(workgroup_atomic_arr[1], 1);
    atomicMax(workgroup_struct.atomic_scalar, 1u);
    atomicMax(workgroup_struct.atomic_arr[1], 1);
    memoryBarrierShared();
    barrier();
    atomicMin(_group_0_binding_0_cs, 1u);
    atomicMin(_group_0_binding_1_cs[1], 1);
    atomicMin(_group_0_binding_2_cs.atomic_scalar, 1u);
    atomicMin(_group_0_binding_2_cs.atomic_arr[1], 1);
    atomicMin(workgroup_atomic_scalar, 1u);
    atomicMin(workgroup_atomic_arr[1], 1);
    atomicMin(workgroup_struct.atomic_scalar, 1u);
    atomicMin(workgroup_struct.atomic_arr[1], 1);
    memoryBarrierShared();
    barrier();
    uint _e163 = atomicAnd(_group_0_binding_0_cs, 1u);
    int _e167 = atomicAnd(_group_0_binding_1_cs[1], 1);
    uint _e171 = atomicAnd(_group_0_binding_2_cs.atomic_scalar, 1u);
    int _e176 = atomicAnd(_group_0_binding_2_cs.atomic_arr[1], 1);
    uint _e179 = atomicAnd(workgroup_atomic_scalar, 1u);
    int _e183 = atomicAnd(workgroup_atomic_arr[1], 1);
    uint _e187 = atomicAnd(workgroup_struct.atomic_scalar, 1u);
    int _e192 = atomicAnd(workgroup_struct.atomic_arr[1], 1);
    memoryBarrierShared();
    barrier();
    uint _e195 = atomicOr(_group_0_binding_0_cs, 1u);
    int _e199 = atomicOr(_group_0_binding_1_cs[1], 1);
    uint _e203 = atomicOr(_group_0_binding_2_cs.atomic_scalar, 1u);
    int _e208 = atomicOr(_group_0_binding_2_cs.atomic_arr[1], 1);
    uint _e211 = atomicOr(workgroup_atomic_scalar, 1u);
    int _e215 = atomicOr(workgroup_atomic_arr[1], 1);
    uint _e219 = atomicOr(workgroup_struct.atomic_scalar, 1u);
    int _e224 = atomicOr(workgroup_struct.atomic_arr[1], 1);
    memoryBarrierShared();
    barrier();
    uint _e227 = atomicXor(_group_0_binding_0_cs, 1u);
    int _e231 = atomicXor(_group_0_binding_1_cs[1], 1);
    uint _e235 = atomicXor(_group_0_binding_2_cs.atomic_scalar, 1u);
    int _e240 = atomicXor(_group_0_binding_2_cs.atomic_arr[1], 1);
    uint _e243 = atomicXor(workgroup_atomic_scalar, 1u);
    int _e247 = atomicXor(workgroup_atomic_arr[1], 1);
    uint _e251 = atomicXor(workgroup_struct.atomic_scalar, 1u);
    int _e256 = atomicXor(workgroup_struct.atomic_arr[1], 1);
    uint _e259 = atomicExchange(_group_0_binding_0_cs, 1u);
    int _e263 = atomicExchange(_group_0_binding_1_cs[1], 1);
    uint _e267 = atomicExchange(_group_0_binding_2_cs.atomic_scalar, 1u);
    int _e272 = atomicExchange(_group_0_binding_2_cs.atomic_arr[1], 1);
    uint _e275 = atomicExchange(workgroup_atomic_scalar, 1u);
    int _e279 = atomicExchange(workgroup_atomic_arr[1], 1);
    uint _e283 = atomicExchange(workgroup_struct.atomic_scalar, 1u);
    int _e288 = atomicExchange(workgroup_struct.atomic_arr[1], 1);
    return;
}

