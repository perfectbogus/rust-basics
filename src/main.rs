use std::cmp::max;
use std::i32::MAX;

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

    println!("----------------------------- Arrays -------------------------------------");
    // Compound Data Types
    // Array Data Type
    let letters = ['a', 'b', 'c'];
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter);

    let mut mut_letters = ['a', 'b', 'c'];
    mut_letters[0] = 'X';
    let first_letter = mut_letters[0];
    println!("mut_letter is {}", first_letter);

    //uninitialized array
    //let numbers : [i32; 5];
    //println!("last number is {}", numbers[4])
    // this will cause an error uninitialized array

    println!("--------------- Initialization ------------------------------");
    let numbers: [i32; 5];
    numbers = [0, 0, 0, 0, 0];
    println!("last number is {}", numbers[4]);

    // Short Initializer
    println!("----------------- Short Initialization -----------------------");
    let numbers: [i32; 5] = [0; 5];
    println!("last number is {}", numbers[4]);

    println!("----------------- Out Bound Index in Compile Time --------------------");
    // let numbers: [i32; 5] = [0; 5];
    // println!("last number is {}", numbers[5])

    println!("----------------- Out Bound Index in Runtime -----------------------");
    // let numbers: [i32; 5] = [0; 5];
    // let len = numbers.len();
    // println!("last number is {}", numbers[len])

    println!("----------------- Multidimensional Arrays -----------------------");
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let number = parking_lot[0][1];
    println!("number is {}", number);

    let number = parking_lot[1][1];
    println!("number is {}", number);

    println!("----------------- Multidimensional Definition Arrays -----------------------");
    let parking_lot_multiple: [[[i32; 10]; 10]; 10];
    let garage = [[[0; 10]; 10]; 10];
    println!("array[0][0][0] is {}", garage[0][0][0]);

    println!("----------------- Tuples Data Type -----------------------");
    let stuff = (10, 3.1416, 'X');
    let stuff_2: (i32, f32, char) = (10, 3.1416, 'X'); // Definition with assignation
    let first_item = stuff.0;
    let second_item = stuff.1;
    let third_item = stuff.2;
    println!("first_item: {}", first_item);
    println!("second_item: {}", second_item);
    println!("third_item: {}", third_item);

    println!("------------------ Mutable Tuple Data Type ----------------------");
    let mut stuff = (10, 3.1416, 'X');
    stuff.0 = 5;
    let first_item = stuff.0;
    let second_item = stuff.1;
    let third_item = stuff.2;
    println!("first_item: {}", first_item);
    println!("second_item: {}", second_item);
    println!("third_item: {}", third_item);

    println!("------------------ Deconstruct Tuple Data Type ----------------------");
    let (a, b, c) = stuff;
    println!("a is {}, b is {}, c is {}", a, b ,c);

    println!("---------------------- Diff Statement and Expression -----------------");
    println!("Expression return a value");
    println!("Statement does not return a value");

    println!("--- Program Flow Control ---");
    let x = 3;
    if x == 3 {
        println!("x is 3");
    }

    // You cannot use an integer into if expression
    // if x {} this will cause an error

    // You can use booleans
    // if true {} this is allowed

    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else {
        println!("x is not greater than y");
    }

    println!();
    println!("--- Nested if else ---");
    if x > y {
        println!("x is greater than y");
    } else {
        if x < y {
            println!("x is less than y");
        } else {
            println!("x is equal to y");
        }
    }
    println!();

    println!("--- Improve expressions ---");
    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y")
    }

    println!();
    println!("--- Conditional Assignment ---");
    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    }

    // this cause an error due to you are not assign x in false case
    // if make_x_odd {
    //     x = 1;
    // } else {
    //
    // }

    println!();
    println!("--- Loops ---");

    let mut count = 0;

    loop {
        count += 1;
        println!("count is {}", count);
        if count == 100 {
            break;
        }
    }

    println!();
    println!("--- loop assign with let ---");

    let mut sum = 0;

    let result = loop {
        if sum == 10 {
            break sum * 10;
        }
        sum += 1;
        println!("sum is {}", sum);
    };

    println!("result is {}", result);

    println!();
    println!("--- while ---");

    let mut count = 0;

    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }

    // Note: for loops and while
    // infinite loops
    // let result = loop {
    //      // do something
    //      break value;
    // }
    // this infinite loop allow you return a value even when its infinite

    // while true {
    //      // do something
    //      break;
    // }
    // this infinite loop does not allow return a value

    println!();
    println!("--- For loops ---");
    let message = ['a', 'b', 'c'];

    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'c'{
            break;
        }
    }

    println!();
    println!("--- Range of numbers ---");
    for number in 0..5 {
        println!("number is {}", number);
    }


    println!();
    println!("-- Nested Loops ---");
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter() {
        for num in row.iter() {
            print!("{}\t", num);
        }
        println!();
    }

    println!();
    println!("-- Nested Loops with Mutable Array ---");
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{}\t", num);
        }
        println!();
    }

    println!();
    println!("-- Challenge Program Flow Control ---");
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = i32::MIN;
    let mut min: i32 = i32::MAX;
    let mut mean: f64 = 0.0;

    for &num in numbers.iter() {
        if num > max {
           max = num;
        }

        if num < min {
            min = num;
        }

        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    println!("Max: {}, Min: {}, Mean: {}", max, min, mean);
    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed");

    println!("--- Ownership ---");
    if true {
        let planet = "Earth";
        println!("planet {planet}")
    }
    // if you try to use planet after here, you are going to get an error

    println!("--- shadowing ---");
    let planet = "Earth";
    println!("planet is {planet}");
    let planet = "Mars";
    println!("planet is {planet}");

    let planet = "Earth";
    println!("planet is {planet}");
    let planet = 4;
    println!("planet is {planet}");

    println!();
    println!("--- Risk Shadowing ---");
    let planet = "Earth";
    {
        println!("planet is {planet}");
        let mut planet = 4;
        println!("planet is {planet}");
    }
    println!("planet is {planet}");

    println!();
    println!("--- Stack and Heap Memory ---");
    // Fixed size for variable on the stack
    // Unknown size for variable on the heap

    header("String Data Type");
    //String Literal
    //  Hardcoded
    //  Immutable
    //  Must be known before the compilation

    //String Type
    //  Allocated on the heap
    //  Mutable
    //  Dynamically generated at runtime
    let message = String::from("Earth");
    println!("message {message}");

    // let it mutable
    let mut message = String::from("Earth");
    println!("message: {message}");
    message.push_str(" is home");
    println!("message: {message}");

    header("Ownership");
    // Copy
    //  Done for stack-only data type such as integer and floating point
    //  Copying occurs implicitly; cloning must be done explicitly

    let rocket_fuel = 1;
    process_fuel(rocket_fuel); // you do copy of rocket_fuel due to is a stack variable
    println!("rocket fuel is {rocket_fuel}");

    //let type_fuel = String::from("RP-1");
    //process_type_fuel(type_fuel); //this fail because you are borrow a variable from heap memory
    // and is using after pass to the function
    //println!("type_fuel is {type_fuel}");

    let type_fuel = String::from("RP-1");
    let type_fuel = process_type_fuel(type_fuel);
    println!("type_fuel is {type_fuel}");

    let type_fuel = String::from("RP-1");
    let type_fuel = process_new_fuel(type_fuel);
    println!("new type_fuel is {type_fuel}");
}

fn process_new_fuel(propellant: String) -> String {
    println!("processing propellant {propellant}");
    let new_fuel = String::from("LNG");
    new_fuel
}

fn process_type_fuel(propellant: String) -> String{
    println!("Processing propellant {propellant}");
    propellant
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant {propellant}");
}

fn header(message: &str){
    println!();
    println!("----- {message} ------");
}