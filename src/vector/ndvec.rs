
pub struct VectorFloat32 {
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
            vector: data.clone(),
            dim: data.len() as u16,
            r
        }
    }
}



