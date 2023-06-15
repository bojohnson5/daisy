use std::collections::VecDeque;

use crate::quantity::Quantity;
use crate::parser::Operator;
use crate::parser::Token;
use super::EvalError;
use crate::context::Context;

pub fn eval_operator(op: &Operator, args: &VecDeque<Token>, context: &mut Context) -> Result<Option<Token>, EvalError> {
	match op {

		// Handled seperately in evaluate.rs
		Operator::Function(_) |

		// These are never evaluated,
		// but are converted to one of the following instead.
		Operator::ImplicitMultiply |
		Operator::Sqrt |
		Operator::Divide |
		Operator::DivideLong |
		Operator::Subtract => { panic!() }

		Operator::Define => {
			if args.len() != 2 { panic!() };
			let b = &args[1];

			if let Token::Variable(s) = &args[0] {
				context.push_var(s.clone(), b.clone());
				return Ok(Some(b.clone()));
			} else { return Err(EvalError::BadDefineName); }
		},

		Operator::Negative => {
			if args.len() != 1 { panic!() };
			let args = &args[0];

			if let Token::Quantity(v) = args {
				return Ok(Some(Token::Quantity(-v.clone())));
			} else { return Ok(None); }
		},

		Operator::Flip => {
			if args.len() != 1 { panic!() };
			let args = &args[0];

			if let Token::Quantity(v) = args {
				if v.is_zero() { return Err(EvalError::ZeroDivision); }
				return Ok(Some(Token::Quantity(
					Quantity::new_rational(1f64).unwrap()/v.clone()
				)));
			} else { return Ok(None); }
		},

		Operator::Add => {
			let mut sum: Quantity;
			if let Token::Quantity(s) = &args[0] {
				sum = s.clone();
			} else { return Ok(None); };

			let mut i: usize = 1;
			while i < args.len() {
				let j = &args[i];
				if let Token::Quantity(v) = j {

					if !sum.unit.compatible_with(&v.unit) {
						return Err(EvalError::IncompatibleUnit);
					}

					sum += v.clone();
				} else { return Ok(None); }
				i += 1;
			}
			return Ok(Some(Token::Quantity(sum)));
		},

		Operator::Multiply => {
			let mut prod = Quantity::new_rational(1f64).unwrap();
			for i in args.iter() {
				let j = i;
				if let Token::Quantity(v) = j {
					prod *= v.clone();
				} else { return Ok(None); }
			}
			return Ok(Some(Token::Quantity(prod)));
		},

		Operator::ModuloLong
		| Operator::Modulo => {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {

					if !(va.unitless() && vb.unitless()) {
						return Err(EvalError::IncompatibleUnit);
					}

					if vb <= &Quantity::new_rational(1f64).unwrap() { return Err(EvalError::BadMath); }
					if va.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }
					if vb.fract() != Quantity::new_rational(0f64).unwrap() { return Err(EvalError::BadMath); }

					return Ok(Some(Token::Quantity(va.clone() % vb.clone())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::UnitConvert
		=> {
			if args.len() != 2 { panic!() };
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {
					let n = va.clone().convert_to(vb.clone());
					if n.is_none() {
						return Err(EvalError::IncompatibleUnit);
					}
					return Ok(Some(Token::Quantity(n.unwrap())));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::Power => {
			if args.len() != 2 {panic!()};
			let a = &args[0];
			let b = &args[1];

			if let Token::Quantity(va) = a {
				if let Token::Quantity(vb) = b {

					if !vb.unitless() {
						return Err(EvalError::IncompatibleUnit);
					}

					if va.is_zero() && vb.is_negative() {
						return Err(EvalError::ZeroDivision);
					}

					let p = va.pow(vb.clone());
					if p.is_nan() {return Err(EvalError::BadMath);}
					return Ok(Some(Token::Quantity(p)));
				} else { return Ok(None); }
			} else { return Ok(None); }
		},

		Operator::Factorial => {
			if args.len() != 1 {panic!()};
			let args = &args[0];

			if let Token::Quantity(v) = args {

				if !v.unitless() {
					return Err(EvalError::IncompatibleUnit);
				}

				if !v.fract().is_zero() { return Err(EvalError::BadMath); }
				if v > &Quantity::new_rational(50_000f64).unwrap() { return Err(EvalError::TooBig); }

				let mut prod = Quantity::new_rational(1f64).unwrap();
				let mut u = v.clone();
				while u > Quantity::new_rational(0f64).unwrap() {
					prod *= u.clone();
					u = u - Quantity::new_rational(1f64).unwrap();
				}

				return Ok(Some(Token::Quantity(prod)));
			} else { return Ok(None); }
		}
	};
}