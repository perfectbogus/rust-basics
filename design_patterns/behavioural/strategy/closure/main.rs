struct Adder;
impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8
    {
        f(x, y)
    }
}

fn arith_adder_fn(x: u8, y: u8) -> u8 {
    x + y
}

fn main() {

    let arith_adder = |x, y| { x + y };
    let bool_adder = |x, y| {
        if x == 1 || y == 1 {
            1
        } else {
            0
        }
    };
    let custom_adder = |x, y| 2 * x + y;

    assert_eq!(9, Adder::add(4, 5, arith_adder));
    assert_eq!(0, Adder::add(0, 0, bool_adder));
    assert_eq!(5, Adder::add(1, 3, custom_adder));
    assert_eq!(9, Adder::add(4, 5, arith_adder_fn))

}