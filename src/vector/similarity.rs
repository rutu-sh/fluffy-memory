use crate::vector::ndvec::VectorFloat32;

#[inline(always)]
fn dot_p(v1: &[f32], v2: &[f32]) -> f32 {
    assert_eq!(v1.len(), v2.len());

    let mut res: f32 = 0.0;
    for i in 0..v1.len() {
        res += v1[i] * v2[i];
    }

    res
}

#[inline(always)]
pub fn cosine_sim(v1: &VectorFloat32, v2: &VectorFloat32) -> f32 {

    assert_eq!(v1.dim, v2.dim);

    dot_p(&v1.vector, &v2.vector) / (v1.r * v2.r)

}


#[inline(always)]
pub fn dot_mulps(v1: &[f32], v2: &[f32]) -> f32 {
    assert_eq!(v1.len(), v2.len());

    let len = v1.len();
    let mut sum = 0.0;

    let simd_width = 8;
    let chunks = len / simd_width * simd_width;

    // SIMD portion — friendly for LLVM autovectorization
    for i in (0..chunks).step_by(simd_width) {
        // Using iterators prevents aliasing concerns
        let a = &v1[i..i + simd_width];
        let b = &v2[i..i + simd_width];

        for j in 0..simd_width {
            sum += a[j] * b[j];
        }
    }

    // Remainder loop
    for i in chunks..len {
        sum += v1[i] * v2[i];
    }

    sum
}


#[inline(always)]
pub fn cosine_sim_mulps(v1: &VectorFloat32, v2: &VectorFloat32) -> f32 {

    assert_eq!(v1.dim, v2.dim);

    dot_mulps(&v1.vector, &v2.vector) / (v1.r * v2.r)

}



