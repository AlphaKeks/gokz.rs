pub(crate) mod convert;

macro_rules! is {
	($name:ident, $variant:ident) => {
		#[allow(missing_docs)]
		pub const fn $name(&self) -> bool {
			matches!(self, Self::$variant)
		}
	};
}

pub(crate) use is;
