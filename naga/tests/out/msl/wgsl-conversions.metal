// language: metal1.0
#include <metal_stdlib>
#include <simd/simd.h>

using metal::uint;

constant metal::float2 ic0_ = metal::float2(2.0, 2.0);

[[max_total_threads_per_threadgroup(1)]]
kernel void main_(
) {
    metal::float2 vc0_ = metal::float2(2.0, 2.0);
    metal::float2 lc0_ = metal::float2(2.0, 2.0);
    return;
}
