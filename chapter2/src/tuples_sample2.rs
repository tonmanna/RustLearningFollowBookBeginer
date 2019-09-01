pub fn tuples_sample() {
    let tuple1 = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("First Value {}", tuple1.0);
    println!("Second Value {}", tuple1.1);

    let tuple2 = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Value {:?}", tuple2);

    // let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Error long tuple2 Value {:?}", long_tuple);
    // Can't print long tuple without implement fmt (maximum is 12)

    let tuple3 = (1u8,);
    println!("One apart of tuple {:?}", tuple3);

    let tuple4 = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple4;
    println!("{:?},{:?},{:?},{:?}", a, b, c, d);
}
