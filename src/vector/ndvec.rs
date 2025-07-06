use uuid::Uuid;


#[allow(dead_code)]
pub struct VectorFloat32 {
    pub id: Uuid,
    pub vector: Vec<f32>,
    pub dim: u16,
    pub r: f32,
}


impl VectorFloat32 {
    pub fn new(data: &Vec<f32>) -> VectorFloat32 {
        let mut r: f32 = 0.0;
        
        for &val in data {
            r += val * val;
        }

        r = r.sqrt();

        VectorFloat32{
            id: Uuid::new_v4(),
            vector: data.clone(),
            dim: data.len() as u16,
            r
        }
    }
}


#[test]
fn test_vec_new() {
    let va: Vec<f32> = vec![1.0; 4];
    let v = VectorFloat32::new(&va);
    assert_eq!(v.r, 2.0);
    assert_eq!(v.vector, va);
    assert_eq!(v.dim, 4);
}
