use utility;
fn main() {
    let num = 100u32;
    let bool = true;
    let unit = ();
    let num2 = num;
    println!(" Integer1 : {:?}", num);
    println!(" Integer2 : {:?}", num2);
    println!(" Boolean : {:?}", bool);
    println!(" Unit : {:?}", unit);
    utility::print_seperator();
    mutable_sample();
    utility::print_seperator();
    scope_sample();
    utility::read_key();
}

fn mutable_sample() {
    let _immutable = 2;
    let mut mutable = 2;
    println!("Before mutation: {}", mutable);
    mutable += 1;
    println!("Afer mutation: {}", mutable);
}

fn scope_sample() {
    let scope_1 = 100;
    {
        let scope_1 = 10;
        println!("Scope_1: {}", scope_1);
    }
    println!("Scope_1: {}", scope_1);
}
