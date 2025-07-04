mod vector;

use vector::ndvec::VectorFloat32;
use vector::similarity;
use std::time::Instant;

fn main() {

    println!("running cosine_sim_v");

    let vctr: Vec<f32> = vec![5.0; 1000000];
    let vctr2: Vec<f32> = vec![6.0; 1000000];

    let v1 = VectorFloat32::new(&vctr);
    let v2 = VectorFloat32::new(&vctr2);

    let start = Instant::now();
    let sim = similarity::cosine_sim(&v1, &v2);
    let duration = start.elapsed();

    println!("sim: {}, duration: {:?}", sim, duration);

}
