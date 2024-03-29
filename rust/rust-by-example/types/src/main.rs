#![allow(overflowing_literals)]

fn main() {
    println!("casting");
    casting();
    println!("-------------------------");

    println!("literals");
    literals();
    println!("-------------------------");

    println!("inference");
    inference();
    println!("-------------------------");

    println!("aliasing");
    aliasing();
    println!("-------------------------");
}

fn casting() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose value in 8-bit two's complement representation is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn literals() {
    use std::mem;

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals
    // if no constraint exists, the compiler will use i32 for int, and f64 for float.
    let i = 1;
    let f = 1.0;

    println!("size of 'x' in bytes: {}", mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", mem::size_of_val(&f));
}

fn inference() {
    let elem = 5u8;
    // it just knows that it's a vector of something ('Vec<_>').
    let mut vec = Vec::new();

    // Now the compiler knows that 'vec' is a vector of 'u8's ('Vec<u8>').
    vec.push(elem);

    println!("vec: {:?}", vec);
}

fn aliasing() {
    // types must has UpperCamelCase names, or the compiler will raise a warning.
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds, inches, nanoseconds + inches);
}
