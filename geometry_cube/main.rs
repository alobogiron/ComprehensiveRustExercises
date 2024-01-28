//Calculate the magnitude of a vector
// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(vector: &[f64;3]) -> f64 {
    (vector[0].powf(2.0)+vector[1].powf(2.0)+vector[2].powf(2.0)).sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(vector: &[f64;3]) -> [f64;3] {
    let length = magnitude(vector);
    let mut normalized: [f64;3] = [0.0;3];

    normalized[0] = vector[0]/length;
    normalized[1] = vector[1]/length;
    normalized[2] = vector[2]/length;

    //println!("{:?}", normalized);

    normalized
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    v = normalize(&mut v);
    //println!("{:?}", v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
