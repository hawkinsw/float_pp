use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

struct PrettyPrintableFloat {
    f: f32,
}

#[allow(unused_assignments)]
impl Display for PrettyPrintableFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // make a local variable that we are going to munge
        // as we do our work. We want it as a signed value
        // because we want to be able to use <, > around 0
        // to be able to determine the value of the bit in
        // its leftmost position (as we shift those bits off
        // and gradually work through the bits in memory.
        let mut f_bits = i32::from_le_bytes(self.f.to_le_bytes());

        // Create a variable to hold the string representation
        // of the mantissa.
        let mut mantissa: String = format!("");

        // Create some variables to hold the numbers that go before
        // the radix point and after the radix point.
        let mut pre_radix_point = 0u64;
        let mut post_radix_point: f64 = 0f64;

        // Create a boolean to hold whether the value printed is
        // negative or positive.
        let is_negative = if f_bits < 0 { true } else { false };

        // shift off the sign bit.
        f_bits <<= 1;

        // Create a variable to hold the exponent. This is biased
        // by 127 in ieee754 to account for that, too. Also, cast
        // to u32 so that we can use it in a for loop later.

        let mut exponent = f_bits.to_be_bytes()[0] as u32;

        if exponent == 0 {
            // we are denormalized, all bets are off!
        } else {
            assert!(exponent >= 127);

            // In normalized representation, the pre radix point is always
            // implicitly 1. So, make it so here.
            pre_radix_point = 1;

            // we can now safely assume that there is bias
            exponent = exponent - 127;
        }

        // shift off the exponent bits.
        f_bits <<= 8;

        // Here's the algorithm:
        // For every 0..exponent bits of the bits in the mantissa,
        // shift them into the pre radix point. From above, the pre
        // radix point already contains the implicit 1. So, all we
        // have to do at each step is
        // 1. shift the pre radix point binary representation to the left
        // by one (This is from the shortcut algorithm to convert
        // a binary digit to a decimal digit.)
        // 2. That "makes space" in the pre radix point. That space
        // defaults to 0 (in other words, we default to simply multiplying
        // by 2).
        // 3. (conditionally) set the value in the space to be 1 if the bit
        // that we shifted from the mantissa is a 1.
        // 4. In all cases, put the values into the mantissa string for
        // printing; they are technically part of the mantissa even though
        // we are using them in the pre radix.
        // 5. Shift off the bit that we just considered.
        for _ in 0..exponent {
            // (1), (2)
            pre_radix_point <<= 1;
            // (3)
            if f_bits < 0 {
                // (4)
                mantissa += &format!("1");
                pre_radix_point |= 1;
            } else {
                // (4)
                mantissa += &format!("1");
            }
            // (5)
            f_bits <<= 1;
        }

        // The remaining bits are everything that comes after
        // the radix point. Since they are fractions, use the
        // algorithm for fractional binary representation to convert
        // them to decimal:
        // The first bit after the radix point represents 1/2^1.
        // The second bit after the radix point represents 1/2^2.
        // The third bit after the radix point represents 1/2^3, etc.
        for i in 0..(23 - exponent) {
            if f_bits < 0 {
                mantissa += &format!("1");
                // Because we are counting from 0 in the for loop, we have
                // to add 1 to i in our calculation here (see above for why).
                post_radix_point += 1.0f64 / (2u64.pow(i + 1) as f64);
            } else {
                mantissa += &format!("0");
            }
            f_bits <<= 1;
        }

        // Do the printing.
        let mut result = write!(
            f,
            "{}\n",
            ((pre_radix_point as f64) + post_radix_point)
                * (if is_negative { -1f64 } else { 1f64 })
        );
        result = write!(f, "S/Exponent/Mantissa:\n");
        result = write!(
            f,
            "{}/{:8b}/{}",
            if is_negative { "1" } else { "0" },
            exponent,
            mantissa
        );
        result
    }
}

impl PrettyPrintableFloat {
    fn new(f: f32) -> Self {
        Self { f }
    }
}

fn main() {
    let mut pp = PrettyPrintableFloat::new(-9.4);
    println!("{}", pp);
    pp = PrettyPrintableFloat::new(9.5);
    println!("{}", pp);
    pp = PrettyPrintableFloat::new(10.3);
    println!("{}", pp);
    pp = PrettyPrintableFloat::new(0f32);
    println!("{}", pp);
}
