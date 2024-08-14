use std::str::Chars;

pub struct Interpreter<'a> {
    it: Chars<'a>
}

impl<'a> Interpreter<'a> {
    fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected Symbol '{ch}'"),
            None => panic!("Unexpected String")
        }
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{op}'")
            }
        }
    }


}

fn main() {
    let data_str= "this is a &str";
    let data_string = String::from(data_str);

    let chars = data_str.chars();
    println!("data str: {:?}", data_str);
    println!("data string: {:?}", data_string);
    println!("chars: {:?}", chars);

    let mut intr = Interpreter::new("2+3");
    let mut postfix = String::new();
    println!("postfix (before): {:?}", &postfix);
    intr.interpret(&mut postfix);
    println!("postfix (after): {:?}", &postfix);
    assert_eq!(postfix, "23+");


    intr = Interpreter::new("1-2+3-4");
    postfix.clear();
    intr.interpret(&mut postfix);
    assert_eq!(postfix, "12-3+4-");
}