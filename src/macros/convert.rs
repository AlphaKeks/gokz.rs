macro_rules! from {
	([$($from:ty), +] => $for:ty => |$value:pat_param| $impl:block) => {
		$(impl From<$from> for $for {
			#[allow(clippy::useless_conversion)]
			fn from($value: $from) -> $for {
				$impl
			}
		})*
	};

	($from:ty => [$($for:ty), +] => |$value:pat_param| $impl:block) => {
		$(impl From<$from> for $for {
			#[allow(clippy::useless_conversion)]
			fn from($value: $from) -> $for {
				$impl
			}
		})*
	};

	($from:ty => $for:ty => |$value:pat_param| $impl:block) => {
		impl From<$from> for $for {
			#[allow(clippy::useless_conversion)]
			fn from($value: $from) -> $for {
				$impl
			}
		}
	};
}

pub(crate) use from;

macro_rules! try_from {
	([$($from:ty), +] => $for:ty => |$value:pat_param| $impl:block) => {
		$(impl TryFrom<$from> for $for {
			type Error = $crate::Error;

			#[allow(clippy::useless_conversion)]
			fn try_from($value: $from) -> $crate::Result<$for> {
				$impl
			}
		})*
	};

	($from:ty => [$($for:ty), +] => |$value:pat_param| $impl:block) => {
		$(impl TryFrom<$from> for $for {
			type Error = $crate::Error;

			#[allow(clippy::useless_conversion)]
			fn try_from($value: $from) -> $crate::Result<$for> {
				$impl
			}
		})*
	};

}

pub(crate) use try_from;
