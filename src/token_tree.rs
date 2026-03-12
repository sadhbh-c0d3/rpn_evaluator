use std::{mem, str::Chars};

pub enum TokenTree {
    Token(String),
    Branch(Vec<TokenTree>),
}

pub fn parse_expression<'a>(
    chars: &mut Chars<'a>,
    is_nested: bool,
) -> Result<TokenTree, &'static str> {
    let mut current_str = String::new();
    let mut tokens = Vec::new();
    let mut expect_comma = false;
    let mut expect_brace = is_nested;

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if !current_str.is_empty() {
                expect_comma = true;
            }
            continue;
        }

        match c {
            '(' => {
                if !current_str.is_empty() {
                    return Err("Unexpected '('");
                }
                let tmp_tree = parse_expression(chars, true)?;
                tokens.push(tmp_tree);
                expect_comma = true;
            }
            ')' => {
                if !expect_brace {
                    return Err("Unexpected ')'");
                }
                expect_brace = false;
                break;
            }
            ',' => {
                if !current_str.is_empty() {
                    tokens.push(TokenTree::Token(mem::take(&mut current_str)));
                }
                expect_comma = false;
            }
            c => {
                if expect_comma {
                    return Err("Expected ','");
                }
                current_str.push(c);
            }
        }
    }

    if expect_brace {
        return Err("Expected ')'");
    }

    if tokens.is_empty() && !is_nested {
        Ok(TokenTree::Token(current_str))
    } else {
        if !current_str.is_empty() {
            tokens.push(TokenTree::Token(mem::take(&mut current_str)));
        }
        Ok(TokenTree::Branch(tokens))
    }
}
