use std::{fmt::Display, iter::Peekable, str::Chars};

// 自定义 Result 类型
pub type Result<T> = std::result::Result<T, ExprError>;

// 自定义错误类型
#[derive(Debug)]
pub enum ExprError {
    Parse(String),
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}

// Token 表示，数字、运算符号、括号
#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i64),
    Plus,       // 加
    Minus,      // 减
    Multiply,   // 乘
    Divide,     // 除
    Power,      // 幂
    Modulo,     // 新增：取模 %
    BitwiseAnd, // 新增：按位与 &
    LeftParen,  // 左括号
    RightParen, // 右括号
}

// 左结合
const ASSOC_LEFT: i32 = 0;
// 右结合
const ASSOC_RIGHT: i32 = 1;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Plus => "+".to_string(),
                Token::Minus => "-".to_string(),
                Token::Multiply => "*".to_string(),
                Token::Divide => "/".to_string(),
                Token::Power => "^".to_string(),
                Token::Modulo => "%".to_string(),
                Token::BitwiseAnd => "&".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
            }
        )
    }
}

impl Token {
    // 判断是不是运算符号
    fn is_operator(&self) -> bool {
        match self {
            Token::Plus
            | Token::Minus
            | Token::Multiply
            | Token::Divide
            | Token::Modulo
            | Token::BitwiseAnd
            | Token::Power => true,
            _ => false,
        }
    }

    // 获取运算符号的优先级
    fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide | Token::Modulo => 2,
            Token::Power => 3,
            Token::BitwiseAnd => 1, // & 计算优先级最低
            _ => 0,
        }
    }
    // 获取运算符号的结合性
    fn associativity(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Modulo => {
                ASSOC_LEFT
            }
            Token::Power => ASSOC_RIGHT,
            Token::BitwiseAnd => ASSOC_LEFT,
            _ => 0,
        }
    }

    // 根据当前运算符进行计算
    fn compute(&self, l: i64, r: i64) -> Option<i64> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => Some(l / r),
            Token::Power => Some(l.pow(r as u32)),
            Token::Modulo => {
                if r == 0 {
                    None // 处理除零错误
                } else {
                    Some(l % r)
                }
            }
            Token::BitwiseAnd => Some(l & r),
            _ => None,
        }
    }
}

// 将一个算术表达式解析成连续的 Token
// 并通过 Iterator 返回，也可以通过 Peekable 接口获取
struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(expr: &'a str) -> Self {
        Self {
            tokens: expr.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    // 扫描数字
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric() {
                num.push(c);
                self.tokens.next();
            } else {
                break;
            }
        }

        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None,
        }
    }

    // 扫描运算符号
    fn scan_operator(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('%') => Some(Token::Modulo),
            Some('&') => Some(Token::BitwiseAnd),
            _ => None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        // 消除前面的空格
        self.consume_whitespace();

        match self.tokens.peek() {
            Some(&c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => return None,
        }
    }
}

pub struct Expr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(expr).peekable(),
        }
    }

    //计算单个Token或者子表达式
    // 计算单个 Token或者子表达式
    fn compute_atom(&mut self) -> Result<i64> {
        // 先检查是否为一元负号
        let negative = match self.iter.peek() {
            Some(Token::Minus) => {
                self.iter.next();
                true
            }
            // 处理连续负号的情况（如--5 = 5）
            Some(Token::Plus) => {
                self.iter.next();
                false
            }
            _ => false,
        };

        let mut value = match self.iter.peek() {
            // 如果是数字的话，直接返回
            Some(Token::Number(n)) => {
                let val = *n;
                self.iter.next();
                // return Ok(val);
                val
            }
            // 如果是左括号的话，递归计算括号内的值
            Some(Token::LeftParen) => {
                self.iter.next();
                let result = self.compute_expr(1)?;
                match self.iter.next() {
                    Some(Token::RightParen) => (),
                    _ => return Err(ExprError::Parse("Unexpected character".into())),
                }
                return Ok(result);
            }
            _ => {
                return Err(ExprError::Parse(
                    "Expecting a number or left parenthesis".into(),
                ))
            }
        };

        // 应用一元负号
        if negative {
            value = -value;
        }
        Ok(value)
    }

    fn compute_expr(&mut self, min_prec: i32) -> Result<i64> {
        // 计算第一个 Token
        let mut atom_lhs = self.compute_atom()?;

        loop {
            let cur_token = self.iter.peek();
            if cur_token.is_none() {
                break;
            }
            let token = *cur_token.unwrap();

            // 1. Token 一定是运算符
            // 2. Token 的优先级必须大于等于 min_prec
            if !token.is_operator() || token.precedence() < min_prec {
                break;
            }

            let mut next_prec = token.precedence();
            if token.associativity() == ASSOC_LEFT {
                next_prec += 1;
            }

            self.iter.next();

            // 递归计算右边的表达式
            let atom_rhs = self.compute_expr(next_prec)?;

            // 得到了两边的值，进行计算
            match token.compute(atom_lhs, atom_rhs) {
                Some(res) => atom_lhs = res,
                None => return Err(ExprError::Parse("Unexpected expr".into())),
            }
        }
        Ok(atom_lhs)
    }

    // 计算表达式，获取结果
    pub fn eval(&mut self) -> Result<i64> {
        let result = self.compute_expr(1)?;
        // 如果还有 Token 没有处理，说明表达式存在错误
        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpected end of expr".into()));
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calc() {
        // let mut expr = Expr::new("1+2*3");
        // let result = expr.eval();
        // assert_eq!(result.unwrap(), 7);
        // let mut expr = Expr::new("1+2*3+4x");
        // let result = expr.eval();
        // assert_eq!(result.unwrap(), 11);
        // 取模运算
        let mut expr = Expr::new("7%4");
        assert_eq!(expr.eval().unwrap(), 3);

        // 按位与运算
        let mut expr = Expr::new("6&3"); // 0110 & 0011 = 0010
        assert_eq!(expr.eval().unwrap(), 2);

        // 混合运算
        let mut expr = Expr::new("(15%8)&7"); // 7 & 7 = 7
        assert_eq!(expr.eval().unwrap(), 7);

        // 错误处理测试
        let mut expr = Expr::new("5%0");
        assert!(expr.eval().is_err());

        // 括号内的负数
        let mut expr = Expr::new("(-5 + 2) * 3");
        assert_eq!(expr.eval().unwrap(), -9); // (-3) * 3 = -9

        // 与幂运算结合
        let mut expr = Expr::new("-2^3");
        assert_eq!(expr.eval().unwrap(), -8); // -(2^3) = -8
    }
}
