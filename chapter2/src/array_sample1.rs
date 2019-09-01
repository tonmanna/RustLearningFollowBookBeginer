use std::mem;

pub fn analyze_slice(slice: &[i32]) {
    println!("first element {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

pub fn array_sample1() {
    let array1: [i32; 5] = [1, 2, 3, 4, 5];
    let array2: [i32; 500] = [0; 500];
    println!("First element 1 : {}", array1[0]);
    println!("Second element 1 : {}", array1[1]);
    println!("Array element length : {}", array1.len());
    println!("Size in memory : {} bytes", mem::size_of_val(&array2));
    analyze_slice(&array2);
    analyze_slice(&array2[1..4]);
    // not allow compile error
    //println!("{}", array1[5]);
}
