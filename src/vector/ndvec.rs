
#[allow(dead_code)]
pub struct VectorFloat32 {
    pub unit_vec: Vec<f32>,
    pub dim: u16,
}


#[allow(dead_code)]
impl VectorFloat32 {
    pub fn new(data: &Vec<f32>) -> VectorFloat32 {
        let mut unit_vec: Vec<f32> = vec![0.0; data.len()]; 
        let mut r: f32 = 0.0;
        
        for &val in data {
            r += val * val;
        }

        r = r.sqrt();

        for i in 0..data.len() {
            unit_vec[i] = data[i] / r;
        }


        VectorFloat32{
            unit_vec,
            dim: data.len() as u16
        }

    }
}



