use crate::token::Token;

pub trait Visitor<T: Expression<T>> {
    fn visit_binary_expr(&self, expr: &Binary<T>) -> T;
    fn visit_assign_expr(&self, expr: &Assign<T>) -> T;
    fn visit_call_expr  (&self, expr: &Call<T>)   -> T;
}

pub trait Expression<T> { 
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}

// ASSIGNMENT //

pub struct Assign<T: Expression<T>> {
    name: Token,
    value: T,
}

impl<T: Expression<T>> Assign<T> {
    pub fn new(name: Token, value: T) -> Self {
        Self { name, value }
    }
}

impl<T: Expression<T>> Expression<T> for Assign<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_assign_expr(self)
    }
}

// BINARY // 

pub struct Binary<T: Expression<T>> {
    left: T,
    operator: Token,
    right: T,
}

impl<T: Expression<T>> Binary<T> {
    pub fn new(left: T, operator: Token, right: T) -> Self {
        Self { left, operator, right }
    }
}

impl<T: Expression<T>> Expression<T> for Binary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

// CALL //

pub struct Call<T: Expression<T>> {
    callee: T,
    paren: Token,
    arguments: Vec<T>,
}

impl<T: Expression<T>> Call<T> {
    pub fn new(callee: T, paren: Token, arguments: Vec<T>) -> Self {
        Self { callee, paren, arguments }
    }
}

impl<T: Expression<T>> Expression<T> for Call<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_call_expr(self)
    }
}

