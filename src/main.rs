mod vector;
mod collection;

use vector::ndvec::VectorFloat32;
use vector::similarity;

fn main() {

    println!("running cosine_sim");

    let vctr: Vec<f32> = vec![5.0; 1000000];
    let vctr2: Vec<f32> = vec![6.0; 1000000];

    let v1 = VectorFloat32::new(&vctr);
    let v2 = VectorFloat32::new(&vctr2);
    let sim = similarity::cosine_sim(&v1, &v2);
    println!("sim = {}", sim);

}
