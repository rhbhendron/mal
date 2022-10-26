use std::io::{stdin, stdout};

fn READ(input_string: String) -> String {
    input_string
}

fn EVAL(input_string: String) -> String {
    input_string
}
fn PRINT(input_string: String) -> String {
    input_string
}

fn rep(input_string: String) -> String {
    let r_out = READ(input_string);
    let e_out = EVAL(r_out);
    PRINT(e_out)
}

fn main() {
    loop {
        print!("user> ");
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        if let Some('$') = s.chars().next_back() {
            break;
        }
        print!("{}", rep(s));
    }
}
