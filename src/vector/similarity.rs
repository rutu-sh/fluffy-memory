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
