macro_rules! from {
	($from:ty => [$($for:ty), +] => |$value:ident| $impl:block) => {
		$(impl From<$from> for $for {
			fn from($value: $from) -> $for {
				$impl
			}
		})*
	};
}

pub(crate) use from;

macro_rules! try_from {
	([$($from:ty), +] => $for:ty => |$value:pat_param| $impl:block) => {
		$(impl TryFrom<$from> for $for {
			type Error = $crate::Error;

			fn try_from($value: $from) -> $crate::Result<$for> {
				$impl
			}
		})*
	};
}

pub(crate) use try_from;
