/*
 * parser.rs
 * 
 * Rooney Malladi
 */

use crate::defs::*;

fn token_to_astop(tok: TokenKind) -> ASTNodeOp {
    match tok {
        TokenKind::TPlus => ASTNodeOp::Add,
        TokenKind::TMinus => ASTNodeOp::Subtract,
        TokenKind::TStar => ASTNodeOp::Multiply,
        TokenKind::TSlash => ASTNodeOp::Divide,
        _ => {
            panic!("Unknown token {:?}.", tok)
        }
    } 
}