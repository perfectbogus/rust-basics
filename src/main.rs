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

    // Boolean Data Type and Operations
    let a: bool = true;
    let b: bool = false;

    println!("a is {} and b is {}", a, b);
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c = (a ^ b) | (a & b);
    println!("c is {}", c);

    // Similar to lazy evaluation in boolean expressions
    // AND
    // false & true  -> false
    // false & false -> false
    // so you only need to look at the left section
    // OR
    // true | false -> true
    // true | true  -> true
    // so you only need to evaluate the left side

    // Short-Circuiting Logical Operations
    // false && [not evaluated] -> false
    // true || [not evaluated]  -> true
    let c = (a ^ b) || (a & b);
    println!("c is {}", c);

    // Use panic macro to validate Short-Circuiting Logical
    let c = (a ^ b) || panic!();
    println!("c is {}", c);

    // Use panic macro to validate Short-Circuiting Logical
    // this case will execute the panic macro
    //let c = (a ^ b) && panic!();
    //println!("c is {}", c);

    println!();
    println!("--- Comparison Operations ---");

    let a = 1;
    let b = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THEN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
    println!();

    println!();
    println!("--- Comparison Operations using Booleans ---");

    let a = true;
    let b = false;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THEN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
    println!();

    // Notes:
    // You can not compare different data types, just same date type
    // Validate Changes

    // Char Data Type
    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!();
    println!("{}\n{}\n{}", letter, number, finger);

    //Challenge: Find Average
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let sum = a as f64 + b + c as f64;
    let avg = sum / 3.0;

    assert_eq!(avg, 45.1);
    println!("Test passed!");
}