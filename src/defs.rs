/*
 * defs.rs
 * 
 * Varun Malladi
 */

/**** Token ****/

pub struct Token {
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind) -> Token {
        Token { kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    EOF,
    TPlus,
    TMinus,
    TStar,
    TSlash,
    TIntLit(u32),
}

pub struct ASTNode {
    op: ASTNodeOp,
    left: Option<Box<ASTNode>>,
    right: Option<Box<ASTNode>>,
}

pub enum ASTNodeOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    IntLit(u32),
}

impl ASTNode {
    // ---- Build and return heap-allocated ASTNode
    pub fn new(op: ASTNodeOp, left: Option<Box<ASTNode>>, right: Option<Box<ASTNode>>) -> Box<ASTNode> {
        Box::new(
            ASTNode { op, left, right }
        )
    }

    // ---- Make AST leaf node
    pub fn new_leaf(op: ASTNodeOp) -> Box<ASTNode> {
        ASTNode::new(op, None, None)
    }

    // ---- Make ASTNode with one child
    pub fn new_unary(op: ASTNodeOp, left: Box<ASTNode>) -> Box<ASTNode> {
        ASTNode::new(op, Some(left), None)
    }
}