mod array_sample1;
mod tuples_sample1;
mod tuples_sample2;
mod tuples_sample3;
mod type_sample;
use utility;

fn main() {
    type_sample::sample1();
    utility::print_seperator();
    let sample_reverse = tuples_sample1::tuples_sample_reverse((100, true));
    println!("Tuples Example 1 {:?}", sample_reverse);
    utility::print_seperator();
    tuples_sample2::tuples_sample();
    utility::print_seperator();
    tuples_sample3::tuples_sample_struct();
    utility::print_seperator();
    array_sample1::array_sample1();
    utility::print_seperator();
    utility::read_key();
}
