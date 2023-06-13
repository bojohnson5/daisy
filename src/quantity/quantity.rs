use std::ops::{
	Add, Sub, Mul, Div,
	Neg, Rem,

	AddAssign, SubAssign,
	MulAssign, DivAssign
};
use std::cmp::Ordering;

use crate::quantity::Unit;
use crate::quantity::FreeUnit;

use crate::quantity::Scalar;

#[derive(Debug)]
#[derive(Clone)]
pub struct Quantity {
	pub scalar: Scalar,
	pub unit: Unit
}



impl ToString for Quantity {
	fn to_string(&self) -> String {
		let n = self.scalar.to_string();
		if self.unitless() { return n; }

		let u = self.unit.to_string();
		if self.is_one() { return u; };

		if self.unit.no_space() {
			return format!("{n}{u}");
		} else {
			return format!("{n} {u}");
		}
	}
}

impl Quantity {
	pub fn to_string_outer(&self) -> String {
		let n = self.scalar.to_string();
		if self.unitless() { return n; }

		let u = self.unit.to_string();
		if self.unit.no_space() {
			return format!("{n}{u}");
		} else {
			return format!("{n} {u}");
		}
	}

	pub fn new_float(f: f64) -> Option<Quantity> {
		let v = Scalar::new_float(f);
		if v.is_none() { return None; }

		return Some(Quantity{
			scalar: v.unwrap(),
			unit: Unit::new()
		});
	}

	pub fn new_rational(f: f64) -> Option<Quantity> {
		let v = Scalar::new_rational(f);
		if v.is_none() { return None; }

		return Some(Quantity{
			scalar: v.unwrap(),
			unit: Unit::new()
		});
	}

	pub fn new_float_from_string(s: &str) -> Option<Quantity> {
		let v = Scalar::new_float_from_string(s);
		if v.is_none() { return None; }

		return Some(Quantity{
			scalar: v.unwrap(),
			unit: Unit::new()
		});
	}

	pub fn new_rational_from_string(s: &str) -> Option<Quantity> {
		let v = Scalar::new_rational_from_string(s);
		if v.is_none() { return None; }

		return Some(Quantity{
			scalar: v.unwrap(),
			unit: Unit::new()
		});
	}

	pub fn from_scalar(s: Scalar) -> Quantity {
		return Quantity{
			scalar: s,
			unit: Unit::new()
		};
	}

	pub fn insert_unit(&mut self, ui: FreeUnit, pi: Scalar) { self.unit.insert(ui, pi) }
	pub fn set_unit(&mut self, u: Unit) { self.unit = u; }


	pub fn convert_to(self, other: Quantity) -> Option<Quantity> {
		if !self.unit.compatible_with(&other.unit) { return None; }

		let fa = self.unit.to_base_factor();
		let fb = other.unit.to_base_factor();

		return Some(self.mul_no_convert(fa).div_no_convert(fb))
	}
}


macro_rules! quant_foward {
	( $x:ident ) => {
		pub fn $x(&self) -> Quantity {
			if !self.unitless() { panic!() }
			Quantity {
				scalar: self.scalar.$x(),
				unit: self.unit.clone()
			}
		}
	}
}

impl Quantity {

	pub fn is_zero(&self) -> bool { self.scalar.is_zero() }
	pub fn is_one(&self) -> bool { self.scalar.is_one() }
	pub fn is_nan(&self) -> bool { self.scalar.is_nan() }
	pub fn is_negative(&self) -> bool { self.scalar.is_negative() }
	pub fn is_positive(&self) -> bool { self.scalar.is_positive() }
	pub fn unitless(&self) -> bool { self.unit.unitless() }
	pub fn unit(&self) -> &Unit { &self.unit }

	quant_foward!(fract);
	quant_foward!(abs);
	quant_foward!(floor);
	quant_foward!(ceil);
	quant_foward!(round);
	quant_foward!(sin);
	quant_foward!(cos);
	quant_foward!(tan);
	quant_foward!(asin);
	quant_foward!(acos);
	quant_foward!(atan);
	quant_foward!(sinh);
	quant_foward!(cosh);
	quant_foward!(tanh);
	quant_foward!(asinh);
	quant_foward!(acosh);
	quant_foward!(atanh);
	quant_foward!(exp);
	quant_foward!(ln);
	quant_foward!(log10);
	quant_foward!(log2);

	pub fn log(&self, base: Quantity) -> Quantity {
		if !self.unitless() { panic!() }
		Quantity {
			scalar: self.scalar.log(base.scalar),
			unit: self.unit.clone()
		}
	}

	pub fn pow(&self, pwr: Quantity) -> Quantity {
		Quantity {
			scalar: self.scalar.pow(pwr.scalar.clone()),
			unit: self.unit.pow(pwr.scalar)
		}
	}
}


impl Quantity {
	pub fn mul_no_convert(self, other: Self) -> Self {
		Quantity {
			scalar: self.scalar * other.scalar,
			unit: self.unit * other.unit
		}
	}

	pub fn mul_assign_no_convert(&mut self, other: Self) {
		self.scalar *= other.scalar;
		self.unit *= other.unit;
	}

	pub fn div_no_convert(self, other: Self) -> Self {
		Quantity {
			scalar: self.scalar / other.scalar,
			unit: self.unit / other.unit
		}
	}

	pub fn div_assign_no_convert(&mut self, other: Self) {
		self.scalar *= other.scalar;
		self.unit *= other.unit;
	}
}


impl Neg for Quantity where {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Quantity {
			scalar: -self.scalar,
			unit: self.unit
		}
	}
}

impl Add for Quantity {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		if !self.unit.compatible_with(&other.unit) { panic!() }

		let mut o = other;
		if self.unit != o.unit {
			o = o.convert_to(self.clone()).unwrap();
		}

		Quantity {
			scalar: self.scalar + o.scalar,
			unit: self.unit
		}
	}
}

impl AddAssign for Quantity where {
	fn add_assign(&mut self, other: Self) {
		if !self.unit.compatible_with(&other.unit) { panic!() }

		let mut o = other;
		if self.unit != o.unit {
			o = o.convert_to(self.clone()).unwrap();
		}

		self.scalar += o.scalar
	}
}

impl Sub for Quantity {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		if !self.unit.compatible_with(&other.unit) { panic!() }

		let mut o = other;
		if self.unit != o.unit {
			o = o.convert_to(self.clone()).unwrap();
		}

		Quantity {
			scalar: self.scalar - o.scalar,
			unit: self.unit
		}
	}
}

impl SubAssign for Quantity where {
	fn sub_assign(&mut self, other: Self) {
		if !self.unit.compatible_with(&other.unit) { panic!() }

		let mut o = other;
		if self.unit != o.unit {
			o = o.convert_to(self.clone()).unwrap();
		}

		self.scalar -= o.scalar;
	}
}


impl Mul for Quantity {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {


		let mut o = other;
		if self.unit != o.unit {
			if o.unit.compatible_with(&self.unit) {
				o = o.convert_to(self.clone()).unwrap()
			} else {
				let cf = self.unit.common_factor(&o.unit);
				if let Some(f) = cf {
					o = o.convert_to(f).unwrap();
				}
			}
		}

		Quantity {
			scalar: self.scalar * o.scalar,
			unit: self.unit * o.unit
		}
	}
}

impl MulAssign for Quantity where {
	fn mul_assign(&mut self, other: Self) {


		let mut o = other;
		if self.unit != o.unit {
			if o.unit.compatible_with(&self.unit) {
				o = o.convert_to(self.clone()).unwrap()
			} else {
				let cf = self.unit.common_factor(&o.unit);
				if let Some(f) = cf {
					o = o.convert_to(f).unwrap();
				}
			}
		}

		self.scalar *= o.scalar;
		self.unit *= o.unit;
	}
}

impl Div for Quantity {
	type Output = Self;

	fn div(self, other: Self) -> Self::Output {

		let mut o = other;
		if self.unit != o.unit {
			if o.unit.compatible_with(&self.unit) {
				o = o.convert_to(self.clone()).unwrap()
			} else {
				let cf = self.unit.common_factor(&o.unit);
				if let Some(f) = cf {
					o = o.convert_to(f).unwrap();
				}
			}
		}

		Quantity {
			scalar: self.scalar / o.scalar,
			unit: self.unit / o.unit
		}
	}
}

impl DivAssign for Quantity where {
	fn div_assign(&mut self, other: Self) {

		let mut o = other;
		if self.unit != o.unit {
			if o.unit.compatible_with(&self.unit) {
				o = o.convert_to(self.clone()).unwrap()
			} else {
				let cf = self.unit.common_factor(&o.unit);
				if let Some(f) = cf {
					o = o.convert_to(f).unwrap();
				}
			}
		}

		self.scalar /= o.scalar;
		self.unit /= o.unit;
	}
}

impl Rem<Quantity> for Quantity {
	type Output = Self;

	fn rem(self, other: Quantity) -> Self::Output {
		if !self.unit.unitless() { panic!() }
		if !other.unit.unitless() { panic!() }

		Quantity {
			scalar: self.scalar % other.scalar,
			unit: self.unit
		}
	}
}

impl PartialEq for Quantity {
	fn eq(&self, other: &Self) -> bool {
		if self.unit != other.unit {false} else {
			self.scalar == other.scalar
		}
	}
}

impl PartialOrd for Quantity {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.unit != other.unit { panic!() }
		self.scalar.partial_cmp(&other.scalar)
	}
}