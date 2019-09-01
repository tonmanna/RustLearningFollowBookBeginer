pub fn sample1() {
    println!("2+3 = {}", 2u32 + 3);
    println!("2-3 = {}", 2i32 - 3);
    println!("false AND true is {}", false && true);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
