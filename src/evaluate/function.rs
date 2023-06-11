use std::collections::VecDeque;

use crate::parser::Token;
use crate::parser::Function;
use crate::parser::Operator;
use super::EvalError;


pub fn eval_function(f: &Function, args: &VecDeque<Token>) -> Result<Token, EvalError> {
	if args.len() != 1 {panic!()};
	let a = &args[0];
	let Token::Quantity(q) = a else {panic!()};

	if !q.unitless() {
		return Err(EvalError::IncompatibleUnit);
	}

	match f {
		Function::Abs => { return Ok(Token::Quantity(q.abs())); },
		Function::Floor => { return Ok(Token::Quantity(q.floor())); },
		Function::Ceil => { return Ok(Token::Quantity(q.ceil())); },
		Function::Round => { return Ok(Token::Quantity(q.round())); },

		Function::NaturalLog => { return Ok(Token::Quantity(q.ln())); },
		Function::TenLog => { return Ok(Token::Quantity(q.log10())); },

		Function::Sin => { return Ok(Token::Quantity(q.sin())); },
		Function::Cos => { return Ok(Token::Quantity(q.cos())); },
		Function::Tan => { return Ok(Token::Quantity(q.tan())); },
		Function::Asin => { return Ok(Token::Quantity(q.asin())); },
		Function::Acos => { return Ok(Token::Quantity(q.acos())); },
		Function::Atan => { return Ok(Token::Quantity(q.atan())); },

		Function::Csc => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.sin())))
				)
			);
		},
		Function::Sec => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.cos())))
				)
			);
		},
		Function::Cot => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.tan())))
				)
			);
		},


		Function::Sinh => { return Ok(Token::Quantity(q.sinh())); },
		Function::Cosh => { return Ok(Token::Quantity(q.cosh())); },
		Function::Tanh => { return Ok(Token::Quantity(q.tanh())); },
		Function::Asinh => { return Ok(Token::Quantity(q.asinh())); },
		Function::Acosh => { return Ok(Token::Quantity(q.acosh())); },
		Function::Atanh => { return Ok(Token::Quantity(q.atanh())); },

		Function::Csch => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.sinh())))
				)
			);
		},
		Function::Sech => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.cosh())))
				)
			);
		},
		Function::Coth => {
			return Ok(
				Token::Operator(
					Operator::Flip,
					VecDeque::from(vec!(Token::Quantity(q.tanh())))
				)
			);
		},

	}
}