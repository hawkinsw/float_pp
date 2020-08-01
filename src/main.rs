fn main() {
    let mut f_i32 = i32::from_be_bytes(6.2f32.to_be_bytes());
    let mut mantissa:String = format!("");

    let sign = if f_i32 < 0 {
        format!("1")
    } else {
        format!("0")
    };

    // shift off the sign bit.
    f_i32<<=1;

    let exponent = f_i32.to_be_bytes()[0];

    // shift off the exponent bits.
    f_i32<<=8;

    for _ in 0..23 {
        if f_i32<0 {
            mantissa += &format!("1")
        } else {
            mantissa += &format!("0");
        }
        f_i32<<=1;
    }

    println!("{}/{:b}/{}", sign, exponent, mantissa);
}
