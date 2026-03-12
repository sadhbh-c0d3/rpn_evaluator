use std::io;

use rpn_evaluator::{
    evaluate_tree::evaluate_tree, simple_vm::SimpleVM, token_tree::parse_expression,
};

fn process_line(line: String) -> Result<(), &'static str> {
    let mut chars = line.chars();
    let token_tree = parse_expression(&mut chars, false)?;
    let mut vm = SimpleVM::new();
    evaluate_tree(&mut vm, token_tree)?;
    let result = vm.outcome()?;
    println!("{}", result);
    Ok(())
}

fn main() {
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if let Err(err) = process_line(line) {
                    eprintln!("Error: {:?}", err);
                }
            }
            Err(_) => return,
        }
    }
}

#[cfg(test)]
mod test {
    use rpn_evaluator::{
        evaluate_tree::evaluate_tree, simple_vm::SimpleVM, token_tree::parse_expression,
    };
    use rust_decimal::dec;

    #[test]
    fn test_expressions() {
        let cases = [
            ("1,1,+", dec!(2)),
            ("2,3,4,+,*", dec!(14)),
            ("2,3,+,1,2,+,*", dec!(15)),
            ("2,(1,1,1,U),*", dec!(6)),
            ("(2,3,+),(5,8,*),P", dec!(200)),
            ("2,3,+,(5,8,*,3,6,*,7,3,*,U),*", dec!(395)),
            ("2,3,+,(5,(3,3,2,U),*),*", dec!(200)),
        ];

        for (input, expected) in cases {
            let mut chars = input.chars();
            let token_tree = parse_expression(&mut chars, false).unwrap();
            let mut vm = SimpleVM::new();
            evaluate_tree(&mut vm, token_tree).unwrap();
            let result = vm.outcome().unwrap();
            assert_eq!(result, expected);
        }
    }
}
