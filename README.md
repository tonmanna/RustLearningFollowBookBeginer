Chapter 1 : Getting Started with Rust

    Rust is almost simlar C++ more readbilit and reliable.
    Hello Word project
        cargo new helloword

    Cargo
    cargo is like project management tools + package management similar yarn.

    How Install Rust see more detail at website
        I'm working on windows 10 it can be run rust without any problem.

    Check Rust Version
    rustc --version

    Uninstall
    rustup self uninstall

    Run helloword code
    rustc hello.rs

Chapter 2 : Primitives

    Type
    u32 , i32 , bool , String , 0x

    Tuples
    A tuples store value of diffrent type can be used at Param and Return.

    Struct
    can be used at tuples members.

    // let long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Error long tuple2 Value {:?}", long_tuple);
    // Can't print long tuple without implement fmt (maximum is 12)


    Array [T;size] array is unlike Tuples is allow only sametype
    index begin at 0 same with C++
    Slice &[T] rust allow you can slice array by variable[0..4]
    array.len() array length
    mem::size_of_val can check size in memory of object

    Rust not allow to used Array out of index at compile time by default
    // not allow compile error
    //println!("{}", array1[5]);
