use std;

pub fn fast_cbrt(x: f32) -> f32 {
    let i0: u32 = unsafe { std::mem::transmute(x) };
    let i1 = i0 / 4 + i0 / 16;
    let i2 = i1 + i1 / 16;
    let i3 = i2 + i2 / 256;
    let j = 0x2a511cd0 + i3;
    unsafe { std::mem::transmute(j) }
}

// float fast_pow(float x, float n) {
//     long bit = (*((long*)&x))*n + 0x3f800000*(1-n);
//     return *((float*)&bit);
// }
