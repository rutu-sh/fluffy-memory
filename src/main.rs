mod vector;

use vector::ndvec::VectorFloat32;
use vector::similarity;

fn main() {

    println!("Hello, world!");

    if is_x86_feature_detected!("avx2") {

        let vctr: Vec<f32> = vec![1.0, 2.0, 3.0];
        let vctr2: Vec<f32> = vec![1.0, 2.0, 3.0];

        let v1 = VectorFloat32::new(&vctr);
        let v2 = VectorFloat32::new(&vctr2);

        unsafe {
            let res = similarity::cosine_sim_avx2(&v1.unit_vec, &v2.unit_vec);
            println!("res = {}", res);
        }
    } else {
        println!("avx2 not available");
    }



}
