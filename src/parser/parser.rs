use super::token::{self, Token, TokenIter};
use super::parse_error::ParseError;
use super::macros::expect_token;
use crate::expr::Expr;

type IsConst = bool;

pub fn parse(tokens: Vec<Token>) -> Expr {
    let mut tokens = tokens.into_iter().peekable();
    parse_expr(&mut tokens).0  
}

fn parse_expr(tokens: &mut TokenIter) -> (Expr, IsConst) {
    parse_sum(tokens)
}

fn parse_sum(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (mut lhs, mut is_lhs_const) = parse_product(tokens);

    while let Some(Token::Plus | Token::Minus) = tokens.peek() {
        expect_token!(token in ITER tokens);
        
        let (rhs, is_rhs_const) = parse_product(tokens);
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = merge_to_bin_op(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
    }

    (lhs, is_lhs_const)
}

fn parse_product(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (mut lhs, mut is_lhs_const) = parse_atom(tokens);

    while let Some(Token::Star | Token::Slash) = tokens.peek() {
        expect_token!(token in ITER tokens);
        
        let (rhs, is_rhs_const) = parse_atom(tokens);
        let is_both_const = is_lhs_const && is_rhs_const;
        lhs = merge_to_bin_op(&token, lhs, rhs, is_both_const);
        is_lhs_const = is_both_const;
    }

    (lhs, is_lhs_const)
}

fn parse_atom(tokens: &mut TokenIter) -> (Expr, IsConst) {
    let (lhs, is_lhs_const) = match tokens.peek().unwrap() {
        Token::LParen => parse_parens(tokens),

        Token::Number(_) => {
            expect_token!(Token::Number(n) in ITER tokens);
            (Expr::Num(n), true)
        },

        Token::Ident(_) => {
            expect_token!(Token::Ident(s) in ITER tokens);
            (Expr::Var(s), false)
        },

        Token::Sin => {
            expect_token!(Token::Sin in ITER tokens);
            let (inner, is_inner_const) = parse_parens(tokens);
            let sin = Expr::new_sin(inner);

            if is_inner_const {
                (Expr::Num(sin.eval_const()), true)
            } else {
                (sin, false)
            }
        }

        _ => panic!("Unexpected token {:?}", tokens.peek().unwrap()),
    };

    parse_implicit_multiplication(lhs, is_lhs_const, tokens)
}


// This merges two expressions into a binary operation expression.
// If both expressions are constant, the operation is evaluated and the result is returned as a constant expression.
// Otherwise, the operation is returned as an expression.
// if the token is not a binary operator, this function panics.
fn merge_to_bin_op(
    token: &Token,
    lhs: Expr,
    rhs: Expr,
    is_const: bool,
) -> Expr {
    if is_const {
        op_token_apply_unchecked(token, lhs, rhs).into()
    } else {
        op_token_to_expr_unchecked(token, lhs, rhs)
    }
}

fn op_token_apply_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> f32 {
    let lhs = lhs.eval_const();
    let rhs = rhs.eval_const();
    match token {
        Token::Plus  => lhs + rhs,
        Token::Minus => lhs - rhs,
        Token::Star  => lhs * rhs,
        Token::Slash => lhs / rhs,
        _ => panic!("Unexpected token"),
    }
}

fn op_token_to_expr_unchecked(token: &Token, lhs: Expr, rhs: Expr) -> Expr {
    match token {
        Token::Plus     => Expr::new_add(lhs, rhs),
        Token::Minus    => Expr::new_sub(lhs, rhs),
        Token::Star     => Expr::new_mul(lhs, rhs),
        Token::Slash    => Expr::new_div(lhs, rhs),
        _ => panic!("Unexpected token {:?}", token),
    }
}

// Parses expressions without operator between them.
// All expressions are taken as they are multiplied.
// TODO: doesn't handle sinus
fn parse_implicit_multiplication(mut lhs: Expr, mut is_lhs_const: IsConst, tokens: &mut TokenIter) -> (Expr, IsConst) {
    while let Some(token) = tokens.peek() {
        match token {
            Token::Ident(_) => {
                expect_token!(Token::Ident(s) in ITER tokens);
    
                lhs = Expr::new_mul(lhs, Expr::Var(s));
                is_lhs_const = false;
            },
    
            Token::LParen => {
                let (inner, is_inner_const) = parse_parens(tokens);

                lhs = Expr::new_mul(lhs, inner);
                is_lhs_const = is_lhs_const && is_inner_const;
            },
    
            _ => break,
        }
    }

    (lhs, is_lhs_const)
}

fn parse_parens(tokens: &mut TokenIter) -> (Expr, IsConst) {
    expect_token!(Token::LParen in ITER tokens);
    let (inner, is_inner_const) = parse_expr(tokens);
    expect_token!(Token::RParen in ITER tokens);
    (inner, is_inner_const)
}