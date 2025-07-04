// highly optimized cosine similarity
//
// 1. make cosine similarity of two vectors as fast as possible
// 2. make batch multiplication operation as fast as possible

/*
 * src: 
 *
 * 1. https://www.nickwilcox.com/blog/autovec/
 * 2. https://www.nickwilcox.com/blog/autovec2/
 * 3. https://github.com/arduano/simdeez
 *
 *
 * SIMD: 
 *  ps = packed single precision (f32)
 *  pd = packed double precision (f64)
 *
 *  __m256 = a 256 bit wide data struct for 8 (32 bit) floating point values
 * 
*/


use std::arch::x86_64::*;


pub fn cosine_sim(v1: &[f32], v2: &[f32]) -> f32 {
    assert_eq!(v1.len(), v2.len());

    let mut res: f32 = 0.0;

    for (a, b) in v1.iter().zip(v2.iter()) {
        res += a * b;
    } 

    res 
}


pub fn cosine_sim_v(v1: &[f32], v2: &[f32]) -> f32 {
    assert_eq!(v1.len(), v2.len());

    let mut res: f32 = 0.0;
    for i in 0..v1.len() {
        res += v1[i] * v2[i];
    }

    res
}


pub unsafe fn cosine_sim_avx2(v1: &[f32], v2: &[f32]) -> f32 {

    assert_eq!(v1.len(), v2.len());

    let len = v1.len();
    let chunks = len / 8;
    let mut sum = [0.0f32; 8];

    unsafe {
        let mut acc = _mm256_setzero_ps();

        for i in 0..chunks {

                let idx = i * 8;
                let va = _mm256_loadu_ps(v1.as_ptr().add(idx));
                let vb = _mm256_loadu_ps(v2.as_ptr().add(idx));
                let prod = _mm256_mul_ps(va, vb);
                acc = _mm256_add_ps(acc, prod);
        } 
        
        _mm256_storeu_ps(sum.as_mut_ptr(), acc);
    }
 
    sum.iter().sum::<f32>()

}

