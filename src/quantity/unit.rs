use std::collections::HashMap;
use std::ops::{
	Mul, Div,
	MulAssign, DivAssign
};

use crate::quantity::Scalar;

#[derive(Debug)]
#[derive(Hash)]
#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
pub enum BaseUnit {
	Second,
	Meter,
	Kilogram,
	Ampere,
	Kelvin,
	Mole,
	Candela
}

pub struct CompoundUnit {
	coef_str: &'static str,
	rational: bool,
	units: &'static[(BaseUnit, f64)],
	pub str: &'static str
}

impl CompoundUnit {
	pub const FOOT: CompoundUnit = CompoundUnit {
		coef_str: "0.3048",
		rational: false,
		units: &[(BaseUnit::Meter, 1f64)],
		str: "ft"
	};

	pub fn unit(&self) -> Unit {
		let mut n = Unit::new();
		for (u, p) in self.units.iter() {
			n.insert(*u, *p);
		}
		return n;
	}

	pub fn coef(&self) -> Scalar {
		if self.rational {
			Scalar::new_rational_from_string(self.coef_str).unwrap()
		} else {
			Scalar::new_float_from_string(self.coef_str).unwrap()
		}
	}
}



#[derive(Debug)]
#[derive(Clone)]
pub struct Unit {
	// Unit, power.
	pub val: HashMap<BaseUnit, f64>
}


impl ToString for Unit {
	fn to_string(&self) -> String {
		if self.unitless() { return String::new(); };

		let mut top_empty = true;
		let mut bottom_empty = true;

		for (_, p) in &self.val {
			if *p > 0f64 {
				top_empty = false;
			} else {
				bottom_empty = false;
			}
		};

		let mut t = String::new();
		let mut b = String::new();

		for (u, p) in &self.val {
			let c = match u {
				BaseUnit::Second => "s",
				BaseUnit::Meter => "m",
				BaseUnit::Kilogram => "kg",
				BaseUnit::Ampere => "a",
				BaseUnit::Kelvin => "k",
				BaseUnit::Mole => "mol",
				BaseUnit::Candela => "c"
			};

			if *p == 1f64 {
				t.push_str(&format!("{c}·"));
			} else if *p == -1f64 {
				if top_empty {
					b.push_str(&format!("{c}⁻¹·"));
				} else {
					b.push_str(&format!("{c}·"));
				}
			} else if *p > 0f64 {
				t.push_str(&format!("{c}^{p}·"));
			} else {
				if top_empty {
					b.push_str(&format!("{c}^{}·", p));
				} else {
					b.push_str(&format!("{c}^{}·", -p));
				}
			}
		};

		if top_empty {
			format!("{}", &b[..b.len()-2]) // Slice cuts off the last `·` (2 bytes)
		} else if bottom_empty {
			format!("{}", &t[..t.len()-2])
		} else {
			format!("{}/{}", &t[..t.len()-2], &b[..b.len()-2])
		}
	}
}


impl Unit {

	pub fn new() -> Unit {
		return Unit{
			val: HashMap::new()
		}
	}

	pub fn unitless(&self) -> bool { self.val.len() == 0 }

	pub fn insert(&mut self, u: BaseUnit, p: f64) {
		match self.val.get_mut(&u) {
			Some(i) => {
				let n = *i + p;

				if n == 0f64 {
					self.val.remove(&u);
				} else { *i = n; }
			},
			None => { self.val.insert(u, p); }
		};
	}

	pub fn pow(&self, pwr: f64) -> Unit {
		let mut u = self.clone();
		for (_, p) in &mut u.val {
			*p *= pwr;
		};
		return u;
	}
}


impl PartialEq for Unit {
	fn eq(&self, other: &Self) -> bool {
		for (u, p) in &other.val {
			match self.val.get(u) {
				Some(i) => { if i != p { return false; } },
				None => { return false; }
			};
		}
		return true;
	}
}

impl Mul for Unit {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in &other.val { o.insert(*u, *p); }
		return o;
	}
}

impl MulAssign for Unit where {
	fn mul_assign(&mut self, other: Self) {
		for (u, p) in &other.val { self.insert(*u, *p); }
	}
}

impl Div for Unit {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {
		let mut o = self.clone();
		for (u, p) in &other.val { o.insert(*u, -*p); }
		return o;
	}
}

impl DivAssign for Unit where {
	fn div_assign(&mut self, other: Self) {
		for (u, p) in &other.val { self.insert(*u, -*p); }
	}
}
