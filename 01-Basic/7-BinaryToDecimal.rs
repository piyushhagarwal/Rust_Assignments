// To input binary number from user and convert it into decimal number.

fn binary_to_decimal(mut binary_num: i32) -> i32 {
    let mut decimal_num = 0;
    let mut power = 0;
    let base: i32 = 2;

    while binary_num != 0 {
        let last_bit = binary_num & 1;
        decimal_num += last_bit * base.pow(power);
        power += 1;
        binary_num = binary_num >> 1;  // Use logical right shift here
    }

    decimal_num
}

fn main() {
    println!("{}", binary_to_decimal(0b101));  // Use binary literal with '0b' prefix

}
