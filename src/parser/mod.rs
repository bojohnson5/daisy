mod stage;

mod token;
mod expression;
mod linelocation;

use self::token::Token;

pub use self::{
	expression::Expression,
	expression::Constant,
	expression::Operator,
	expression::Function,
	linelocation::LineLocation,
};

use crate::context::Context;
use crate::errors::DaisyError;

pub fn parse(
	s: &String, context: &Context
) -> Result<Expression, (LineLocation, DaisyError)> {

	let expressions = stage::tokenize(s, context);
	let (_, expressions) = stage::find_subs(expressions);
	let g = stage::groupify(expressions, context)?;
	let g = stage::treeify(g, context)?;

	return Ok(g);
}

pub fn parse_no_context(s: &String) -> Result<Expression, (LineLocation, DaisyError)> {
	parse(s, &Context::new())
}

pub fn substitute(s: &String, context: &Context) -> String {
	let (_, s) = substitute_cursor(s, s.chars().count(), context);
	return s;
}

pub fn substitute_cursor(
	s: &String, // The string to substitute
	c: usize,   // Location of the cursor right now
	context: &Context
) -> (
	usize,  // Location of cursor in substituted string
	String  // String with substitutions
) {
	if s == "" { return (c, s.clone()) }
	let mut new_s = s.clone();

	let l = s.chars().count();
	let expressions = stage::tokenize(s, context);
	let (mut subs, _) = stage::find_subs(expressions);
	let mut new_c = l - c;

	while subs.len() > 0 {
		let r = subs.pop_back().unwrap();
		// Apply substitutions in reverse order

		if { // Don't substitute if our cursor is inside the substitution
			c >= r.0.pos &&
			c < r.0.pos+r.0.len
		} { continue; }

		if c < r.0.pos {
			let ct = r.1.chars().count();
			if ct >= r.0.len {
				if new_c >= ct - r.0.len {
					new_c += ct - r.0.len
				}
			} else {
				new_c -= r.0.len - ct
			}
		}

		new_s.replace_range(
			r.0.pos..r.0.pos+r.0.len,
			&r.1[..]
		)
	}

	return (new_c, new_s);
}