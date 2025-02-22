use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};
#[cfg(feature = "runtime")]
use sp_runtime_interface::pass_by::PassByCodec;

use crate::DataLookup;

pub mod v1;
pub mod v2;

/// Header extension data.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "runtime", derive(PassByCodec))]
pub enum HeaderExtension {
	V1(v1::HeaderExtension),
	V2(v2::HeaderExtension),
}

/// It forwards the call to the inner version of the header. Any invalid version will return the
/// default value or execute an empty block.
macro_rules! forward_to_version {
	($self:ident, $function:ident) => {{
		match $self {
			HeaderExtension::V1(ext) => ext.$function(),
			HeaderExtension::V2(ext) => ext.$function(),
		}
	}};

	($self:ident, $function:ident, $arg:expr) => {{
		match $self {
			HeaderExtension::V1(ext) => ext.$function($arg),
			HeaderExtension::V2(ext) => ext.$function($arg),
		}
	}};
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		forward_to_version!(self, data_root)
	}

	pub fn app_lookup(&self) -> &DataLookup {
		forward_to_version!(self, app_lookup)
	}

	pub fn rows(&self) -> u16 {
		forward_to_version!(self, rows)
	}

	pub fn cols(&self) -> u16 {
		forward_to_version!(self, cols)
	}
}

impl Default for HeaderExtension {
	fn default() -> Self {
		v2::HeaderExtension::default().into()
	}
}

impl From<v1::HeaderExtension> for HeaderExtension {
	#[inline]
	fn from(ext: v1::HeaderExtension) -> Self {
		Self::V1(ext)
	}
}

impl From<v2::HeaderExtension> for HeaderExtension {
	#[inline]
	fn from(ext: v2::HeaderExtension) -> Self {
		Self::V2(ext)
	}
}
