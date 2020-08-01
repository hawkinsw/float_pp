fn main() {
    let f: f32 = 6.2f32;
    let mut f_i32 = i32::from_be_bytes(f.to_be_bytes());

    let sign = if f_i32 < 0 {
        format!("1")
    } else {
        format!("0")
    };
    f_i32<<=1;

    let exponent = f_i32.to_be_bytes()[0];

    f_i32<<=8;

    println!("sign: {}", sign);
    println!("exponent: {:b}", exponent);
    print!("mantissa: ");
    for i in 0..23 {
        if f_i32<0 {
            print!("1");
        } else {
            print!("0");
        }
        f_i32<<=1;
    }
    println!("");
}
