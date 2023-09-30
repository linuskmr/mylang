use crate::source::PositionContainer;
use std::fmt;

pub type Token = PositionContainer<TokenKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
	/// Keyword: Function definition.
	Def,
	/// Function, variable name or data type.
	Identifier(String),
	/// Floating point number.
	Float(f64),
	/// Integer number.
	Int(i64),
	/// Comment (Possible a doc comment)
	Comment(String),
	/// `+`
	Plus,
	/// `*`
	Star,
	/// `-`
	Minus,
	/// `<`
	Less,
	/// `>`
	Greater,
	/// `(`
	OpeningParentheses,
	/// `)`
	ClosingParentheses,
	/// `{`
	OpeningCurlyBraces,
	/// `}`
	ClosingCurlyBraces,
	/// `[`
	OpeningSquareBrackets,
	/// `]`
	ClosingSquareBrackets,
	/// `,`
	Comma,
	/// `;`
	Semicolon,
	/// `:`
	Colon,
	/// `/`
	Slash,
	/// `=`
	Equal,
	/// `=/=`
	NotEqual,
	/// Bitwise OR
	BitOr,
	/// Bitwise AND
	BitAnd,
	/// Modulus %
	Modulus,
	/// If
	If,
	/// Else
	Else,
	/// `while` loop
	While,
	/// `.`
	Dot,
	/// End of line, i.e. `\n`.
	EndOfLine,
	/// `ptr`
	Pointer,
	/// `struct`
	Struct,
	/// `var`
	Var,
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as fmt::Debug>::fmt(self, f)
	}
}
