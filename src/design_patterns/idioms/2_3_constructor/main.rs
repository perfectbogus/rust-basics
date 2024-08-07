//Associated Function
struct Second {
    value: u64
}

impl Second {
    //Associated Function: no self as parameter
    pub fn new(value: u64) -> Self {
        Self {
            value
        }
    }
    pub fn value(&self) -> u64 {
        self.value
    }
}

// Default Constructor using traits
impl Default for Second {
    fn default() -> Self {
        Self {
            value: 0
        }
    }
}

// Using Derive default it is properties had defined default trait
#[derive(Default)]
struct SecondDerive {
    value: u64
}

fn main() {

}