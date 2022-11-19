use std::io::{stdin, stdout};

use brainf_ck::Interpreter;

const PROGRAM: &str = r#"
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
"#;

fn main() {
    let mut interpreter = Interpreter::new(stdin(), stdout());
    if let Err(err) = interpreter.run(PROGRAM) {
        println!("Failed to run: {:?}", err);
        return;
    };
    println!("");
}