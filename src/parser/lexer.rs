use super::macros::char_pat;
use super::parse_error::ParseError;
use super::token::Token;

pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = vec![];
    let mut chars = input.chars().peekable();

    while let Some(&char) = chars.peek() {
        let token = match char {
            ' ' => {
                chars.next();
                continue;
            }

            '+' => {
                chars.next();
                Token::Plus
            }

            '-' => {
                chars.next();
                Token::Minus
            }

            '*' => {
                chars.next();
                Token::Star
            }

            '/' => {
                chars.next();
                Token::Slash
            }

            '^' => {
                chars.next();
                Token::Caret
            }

            '(' => {
                chars.next();
                Token::LParen
            }

            ')' => {
                chars.next();
                Token::RParen
            }

            ',' => {
                chars.next();
                Token::Comma
            }

            '0'..='9' => {
                let mut num_str = String::new();
                parse_digits(&mut num_str, &mut chars);

                if chars.peek() == Some(&'.') {
                    chars.next();
                    num_str.push('.');
                    parse_digits(&mut num_str, &mut chars);

                    if num_str.ends_with('.') {
                        panic!("Number literal must have digits after the decimal point");
                    }
                }

                Token::Number(num_str.parse().unwrap())
            }

            char_pat!(IDENT) => {
                let mut ident_str = String::new();

                parse_ident(&mut ident_str, &mut chars);
                match_keyword(ident_str)
            }

            _ => return Err(ParseError::UnexpectedChar(char)),
        };

        tokens.push(token);
    }

    Ok(tokens)
}

fn parse_digits(string: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    parse_sequence_while(string, chars, |c| c.is_digit(10));
}

fn parse_ident(string: &mut String, chars: &mut std::iter::Peekable<std::str::Chars>) {
    parse_sequence_while(string, chars, |c| matches!(c, char_pat!(IDENT)));
}

fn parse_sequence_while(
    string: &mut String,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    f: fn(char) -> bool,
) {
    while let Some(&char) = chars.peek() {
        if f(char) {
            string.push(char);
            chars.next();
        } else {
            break;
        }
    }
}

fn match_keyword(string: String) -> Token {
    match string.as_str() {
        _ => Token::Ident(string),
    }
}
