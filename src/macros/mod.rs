pub(crate) mod convert;

macro_rules! is {
	($name:ident, $variant:ident) => {
		#[allow(missing_docs)]
		pub const fn $name(&self) -> bool {
			matches!(self, Self::$variant)
		}
	};

	($name:ident, $variant:ident($pat:pat)) => {
		#[allow(missing_docs)]
		pub const fn $name(&self) -> bool {
			matches!(self, Self::$variant($pat))
		}
	};
}

pub(crate) use is;
