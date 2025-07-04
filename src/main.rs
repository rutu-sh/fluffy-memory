mod vector;

use vector::ndvec::VectorFloat32;
use vector::similarity;
use std::time::Instant;

fn main() {

    if is_x86_feature_detected!("avx2") {

        println!("running avx2");
        let start_time = Instant::now();

        let vctr: Vec<f32> = vec![5.0; 1000000];
        let vctr2: Vec<f32> = vec![6.0; 1000000];

        let v1 = VectorFloat32::new(&vctr);
        let v2 = VectorFloat32::new(&vctr2);

        unsafe {
            let res = similarity::cosine_sim_avx2(&v1.unit_vec, &v2.unit_vec);
            println!("res = {}", res);
        }

        let duration = start_time.elapsed();

        println!("avx2 duration: {:?}", duration);

    } 

    println!("running cosine_sim");

    let vctr: Vec<f32> = vec![5.0; 1000000];
    let vctr2: Vec<f32> = vec![6.0; 1000000];

    let v1 = VectorFloat32::new(&vctr);
    let v2 = VectorFloat32::new(&vctr2);

    let start_time = Instant::now();
    let res = similarity::cosine_sim(&v1.unit_vec, &v2.unit_vec);
    println!("res = {}", res);
    let duration = start_time.elapsed();

    println!("cosine-sim duration: {:?}", duration);

    println!("running cosine_sim_v");

    let vctr: Vec<f32> = vec![5.0; 1000000];
    let vctr2: Vec<f32> = vec![6.0; 1000000];

    let v1 = VectorFloat32::new(&vctr);
    let v2 = VectorFloat32::new(&vctr2);

    let start_time = Instant::now();
    let res = similarity::cosine_sim_v(&v1.unit_vec, &v2.unit_vec);
    println!("res = {}", res);
    let duration = start_time.elapsed();

    println!("cosine-sim_v duration: {:?}", duration);

    println!("running cosine_sim_rayon");

    let vctr: Vec<f32> = vec![5.0; 1000000];
    let vctr2: Vec<f32> = vec![6.0; 1000000];

    let v1 = VectorFloat32::new(&vctr);
    let v2 = VectorFloat32::new(&vctr2);

    let start_time = Instant::now();
    let res = similarity::cosine_sim_rayon(&v1.unit_vec, &v2.unit_vec);
    println!("res = {}", res);
    let duration = start_time.elapsed();

    println!("cosine-sim_v duration: {:?}", duration);

}
