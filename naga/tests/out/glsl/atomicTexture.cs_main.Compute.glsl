#version 310 es

precision highp float;
precision highp int;

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

layout(r32ui) uniform highp uimage2D _group_0_binding_0_cs;

layout(r32i) uniform highp iimage2D _group_0_binding_1_cs;


void main() {
    uvec3 id = gl_LocalInvocationID;
    imageAtomicMax(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicMin(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicAdd(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicAnd(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicOr(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicXor(_group_0_binding_0_cs, ivec2(0, 0), 1u);
    imageAtomicMax(_group_0_binding_1_cs, ivec2(0, 0), 1);
    imageAtomicMin(_group_0_binding_1_cs, ivec2(0, 0), 1);
    imageAtomicAdd(_group_0_binding_1_cs, ivec2(0, 0), 1);
    imageAtomicAnd(_group_0_binding_1_cs, ivec2(0, 0), 1);
    imageAtomicOr(_group_0_binding_1_cs, ivec2(0, 0), 1);
    imageAtomicXor(_group_0_binding_1_cs, ivec2(0, 0), 1);
    return;
}

