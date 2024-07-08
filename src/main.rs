fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x += 10;
    println!("x is {}", x);

    let unsigned: u8 = 10;
    println!("x is {}", unsigned);

    //Bitwise Operations
    let value = 0b11110101;
    println!("value is {}", value);

    let value2 = 0b11110101u8;
    println!("value is {:08b}", value2);
    //Bitwise Operations
    // Not
    // or
    // and
    //
    // NOT
    let not_value: u8 = !value2;
    println!("not value is {:08b}", not_value);

    //Bitwise AND
    let and_value: u8 = value2 & 0b11110111;
    println!("and value is {:08b}", and_value);

    println!("    00100000");
    println!("AND 00000000");
    println!("--------------");
    println!("    {:08b}", 0b00100000 & 0b00000000);

    println!("    00100000");
    println!("AND 00100000");
    println!("--------------");
    println!("    {:08b}", 0b00100000 & 0b00100000);

    // Bitwise OR
    println!();
    println!("   00100000");
    println!("OR 00000000");
    println!("--------------");
    println!("   {:08b}", 0b00100000 | 0b00000000);

    println!();
    println!("   00000000");
    println!("OR 00000000");
    println!("--------------");
    println!("   {:08b}", 0b00000000 | 0b00000000);

    // Bitwise XOR
    // 0 0 -> 0
    // 0 1 -> 1
    // 1 0 -> 1
    // 1 1 -> 0
    println!();
    println!("    00000000");
    println!("XOR 00000000");
    println!("--------------");
    println!("    {:08b}", 0b00000000 ^ 0b00000000);

    println!();
    println!("    10000000");
    println!("XOR 10000000");
    println!("--------------");
    println!("    {:08b}", 0b10000000 ^ 0b10000000);

    println!();
    println!("    10000000");
    println!("XOR 00000000");
    println!("--------------");
    println!("    {:08b}", 0b10000000 ^ 0b00000000);

    // BitShift Operator <<
    let shift_left_value = 0b00000001;
    println!("shift left value {:08b}", shift_left_value);
    let shifted_left = shift_left_value << 4;
    println!("shifted    value {:08b}", shifted_left);

    let shift_right = 0b00010000;
    println!("shift right {:08b}", shift_right);
    let shifted_right = shift_right >> 3;
    println!("shifted rig {:08b}", shifted_right);


}