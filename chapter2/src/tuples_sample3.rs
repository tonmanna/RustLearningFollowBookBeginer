#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn tuples_sample_struct() {
    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("{:?}", matrix);
}
