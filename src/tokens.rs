use std::collections::VecDeque;


/// Specifies the location of a token in an input string.
/// Used to locate ParserErrors.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct LineLocation {
	pub pos: usize,
	pub len: usize
}

/// Tokens represent logical objects in an expession.
/// 
/// Tokens starting with `Pre*` are intermediate tokens, and
/// will never show up in a fully-parsed expression tree.
#[derive(Debug)]
pub enum Token {

	/// Used only while tokenizing.
	/// Will be replaced with a Number once we finish.
	PreNumber(LineLocation, String),

	/// Used only while tokenizing.
	/// Will be replaced with one of the Tokens below once we finish.
	PreWord(LineLocation, String),

	/// Used only until operators are parsed.
	/// Each of these will become one of the operators below.
	PreOperator(LineLocation, Operator),

	PreGroupStart(LineLocation),
	PreGroupEnd(LineLocation),
	/// Used only until operators are parsed.
	/// PreGroups aren't needed once we have a tree.
	PreGroup(LineLocation, VecDeque<Token>),

	Number(LineLocation, f64),
	Constant(LineLocation, f64, String),

	Multiply(VecDeque<Token>),
	Divide(VecDeque<Token>),
	Add(VecDeque<Token>),
	Factorial(VecDeque<Token>),
	Negative(VecDeque<Token>),
	Power(VecDeque<Token>),
	Modulo(VecDeque<Token>),
}

impl Token {
	#[inline(always)]
	pub fn get_args(&mut self) -> Option<&mut VecDeque<Token>> {
		match self {
			Token::Multiply(ref mut v)
			| Token::Divide(ref mut v)
			| Token::Add(ref mut v)
			| Token::Factorial(ref mut v)
			| Token::Negative(ref mut v)
			| Token::Power(ref mut v)
			| Token::Modulo(ref mut v)
			=> Some(v),
			_ => None
		}
	}

	#[inline(always)]
	pub fn get_line_location(&self) -> &LineLocation {
		match self {
			Token::PreNumber(l, _) |
			Token::PreWord(l, _) |
			Token::PreOperator(l, _) |
			Token::PreGroupStart(l) |
			Token::PreGroupEnd(l) |
			Token::PreGroup(l, _)
			=> l,

			// These have a line location, but we shouldn't ever need to get it.
			Token::Number(_l, _) |
			Token::Constant(_l, _, _)
			=> panic!(),
			_ => panic!()
		}
	}

	#[inline(always)]
	pub fn get_mut_line_location(&mut self) -> &mut LineLocation {
		match self {
			Token::PreNumber(l, _) |
			Token::PreWord(l, _) |
			Token::PreOperator(l, _) |
			Token::PreGroupStart(l) |
			Token::PreGroupEnd(l) |
			Token::PreGroup(l, _)
			=> l,

			// These have a line location, but we shouldn't ever need to get it.
			Token::Number(_l, _) |
			Token::Constant(_l, _, _)
			=> panic!(),
			_ => panic!()
		}
	}

	#[inline(always)]
	fn as_number(&self) -> Token {
		match self {
			Token::Number(l,v) => {
				Token::Number(*l, *v)
			},
			Token::Constant(l,v,_) => {
				Token::Number(*l, *v)
			},
			_ => panic!()
		}
	}

	pub fn eval(&self) -> Token {
		match self {
			Token::Negative(ref v) => {
				if v.len() != 1 {panic!()};
				let v = v[0].as_number();

				if let Token::Number(l, v) = v {
					Token::Number(l, -v)
				} else { panic!(); }
			},

			Token::Add(ref v) => {
				let mut sum: f64 = 0f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in v.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
						sum += v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					sum
				)
			},

			Token::Multiply(ref v) => {
				let mut prod: f64 = 1f64;
				let mut new_pos: usize = 0;
				let mut new_len: usize = 0;
				for i in v.iter() {
					let j = i.as_number();
					if let Token::Number(l, v) = j {
						if new_pos == 0 {new_pos = l.pos};
						new_len = new_len + l.len;
						prod *= v;
					} else {
						panic!();
					}
				}

				Token::Number(
					LineLocation { pos: new_pos, len: new_len },
					prod
				)
			},

			Token::Divide(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va/vb
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Modulo(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va%vb
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Power(ref v) => {
				if v.len() != 2 {panic!()};
				let a = v[0].as_number();
				let b = v[1].as_number();

				if let Token::Number(la, va) = a {
					if let Token::Number(lb, vb) = b {
						Token::Number(
							LineLocation { pos: la.pos, len: lb.pos - la.pos + lb.len },
							va.powf(vb)
						)
					} else { panic!(); }
				} else { panic!(); }
			},

			Token::Factorial(ref _v) => { todo!() },
			_ => self.as_number()
		}
	}
}


/// Operator types, in order of increasing priority.
/// The Null operator MUST be equal to zero.
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Operator {
	ModuloLong = 0, // Mod invoked with "mod"
	Subtract,
	Add,
	Divide,
	Multiply,
	ImplicitMultiply,
	Modulo, // Mod invoked with %
	Power,

	Negative,
	Factorial,
}

impl Operator {
	#[inline(always)]
	pub fn is_binary(&self) -> bool {
		match self {
			Operator::Negative
			| Operator::Factorial
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	pub fn is_left_associative(&self) -> bool {
		match self {
			Operator::Negative
			=> false,
			_ => true
		}
	}

	#[inline(always)]
	pub fn into_token(&self, mut args: VecDeque<Token>) -> Token {
		match self {
			Operator::Add => Token::Add(args),

			Operator::Multiply
			| Operator::ImplicitMultiply
			=> Token::Multiply(args),
			
			Operator::Subtract => {
				if args.len() != 2 { panic!() }
				let a = args.pop_front().unwrap();
				let b = args.pop_front().unwrap();
	
				Token::Add(
				VecDeque::from(vec!(
						a,
						Token::Negative(VecDeque::from(vec!(b)))
				)))
			},

			Operator::Divide => {
				if args.len() != 2 { panic!() }
				Token::Divide(args)
			},
	
			Operator::ModuloLong |
			Operator::Modulo => {
				if args.len() != 2 { panic!() }
				Token::Modulo(args)
			},
	
			Operator::Power => {
				if args.len() != 2 { panic!() }
				Token::Power(args)
			},

			Operator::Negative => {
				if args.len() != 1 { panic!() }
				Token::Negative(args)
			},

			Operator::Factorial => {
				if args.len() != 1 { panic!() }
				Token::Factorial(args)
			}
		}
	}
}