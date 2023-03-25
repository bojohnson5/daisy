use crate::parser::Token;
use crate::parser::LineLocation;
use crate::parser::ParserError;

#[inline(always)]
fn get_at_coords<'a>(g: &'a mut Token, coords: &Vec<usize>) -> &'a mut Token {
	let mut h = &mut *g;

	for t in coords.iter() {
		let inner = match h {
			Token::Multiply(ref mut v) => v,
			Token::Divide(ref mut v) => v,
			Token::Add(ref mut v) => v,
			Token::Factorial(ref mut v) => v,
			Token::Negative(ref mut v) => v,
			Token::Power(ref mut v) => v,
			Token::Modulo(ref mut v) => v,
			Token::Root(ref mut v) => v,
			_ => panic!()
		};

		h = &mut inner[*t];
	}

	return h;
}


pub fn p_evaluate(
	mut g: Token,
) -> Result<Token, (LineLocation, ParserError)> {
	let mut coords: Vec<usize> = Vec::with_capacity(16);
	coords.push(0);

	'outer: loop {

		let mut h = &mut g;
		for t in coords.iter() {
			let inner = match h {
				Token::Multiply(ref mut v) => v,
				Token::Divide(ref mut v) => v,
				Token::Add(ref mut v) => v,
				Token::Factorial(ref mut v) => v,
				Token::Negative(ref mut v) => v,
				Token::Power(ref mut v) => v,
				Token::Modulo(ref mut v) => v,
				Token::Root(ref mut v) => v,
				_ => panic!()
			};

			if *t >= inner.len() {
				coords.pop();
				if coords.len() == 0 { break 'outer; }


				let p = get_at_coords(&mut g, &coords);
				let e = p.eval();
				*p = e;

				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}

			h = &mut inner[*t];
		}

		match h {
			Token::Multiply(_) |
			Token::Divide(_) |
			Token::Add(_) |
			Token::Factorial(_) |
			Token::Negative(_) |
			Token::Power(_) |
			Token::Modulo(_)
			=> {
				coords.push(0);
				continue 'outer;
			},

			Token::Constant(_,_,_) |
			Token::Number(_,_) => {
				let l = coords.pop().unwrap();
				coords.push(l + 1);
				continue 'outer;
			}

			Token::PreNumber(_,_) |
			Token::PreWord(_,_) |
			Token::PreOperator(_,_) |
			Token::PreGroup(_,_) |
			Token::PreGroupStart(_) |
			Token::PreGroupEnd(_) |
			Token::Root(_)
			=> panic!()
		};
	}


	return Ok(g);
}