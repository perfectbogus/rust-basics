fn main() {
    println!("Hello, world!");
    say_hello();
    say_hello();
    let x = 10;
    let y = 5;
    say_the_sum(x, y);
    println!("-------------- Square Function -----------------");
    square_no_return(3);
    square_return(3);

    println!("------ Tuple ------");
    let square = square_tuple(3);
    println!("value {} and square {}", square.0, square.1);


    println!("--- Challenge ---");
    println!("Convert Celsius to Fahrenheit");
    let celsius: f64 = 15.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("Celsius {} is Fahrenheit {}", celsius, fahrenheit);

    let celsius = 23.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{} Celsius in Fahrenheit {}", celsius, fahrenheit);
    assert_eq!(fahrenheit, 73.4 );
    println!("Test passed");
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (1.8 * celsius) + 32.0
}

// Function Signature
fn say_hello() {
    println!("Hello!");
    say_a_number(13);
}

// Functions with parameter
fn say_a_number(number: i32) {
    println!("number is {}", number);
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("Sum is {}", sum);
}

fn square_no_return(x: i32) -> i32 {
    let square = x * x;
    println!("squaring {}", square);
    square
}

fn square_return(x: i32) -> i32 {
    let square = x * x;
    println!("squaring return explicitly {}", square);
    return square;
}

// Return a Tuple
fn square_tuple(x: i32) -> (i32, i32) {
    let square = x * x;
    println!("squaring return explicitly {}", square);
    return (x, square);
}