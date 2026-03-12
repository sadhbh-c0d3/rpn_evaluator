use super::{simple_vm::SimpleVM, token_tree::TokenTree};

pub fn evaluate_expression(vm: &mut SimpleVM, expr: Vec<TokenTree>) -> Result<(), &'static str> {
    let mut inner_vm = SimpleVM::new();
    for token in expr {
        match token {
            TokenTree::Branch(subexpr) => evaluate_expression(&mut inner_vm, subexpr)?,
            TokenTree::Token(text) => match text.as_str() {
                "+" => {
                    inner_vm.sum(2)?;
                }
                "U" => {
                    inner_vm.sum(inner_vm.depth())?;
                }
                "*" => {
                    inner_vm.product(2)?;
                }
                "P" => {
                    inner_vm.product(inner_vm.depth())?;
                }
                v => {
                    inner_vm.push_operand(v.parse().map_err(|_| "Failed to parse token")?);
                }
            },
        }
    }
    let result = inner_vm.outcome()?;
    vm.push_operand(result);
    Ok(())
}

pub fn evaluate_tree(vm: &mut SimpleVM, tree: TokenTree) -> Result<(), &'static str> {
    match tree {
        TokenTree::Token(token) => {
            vm.push_operand(token.parse().map_err(|_| "Failed to parse operand")?)
        }
        TokenTree::Branch(subexpr) => {
            evaluate_expression(vm, subexpr)?;
        }
    }
    Ok(())
}
