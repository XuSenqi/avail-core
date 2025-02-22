use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::{RuntimeDebug, H256};

use crate::{v2::KateCommitment, DataLookup};

#[derive(PartialEq, Eq, Clone, RuntimeDebug, TypeInfo, Encode, Decode, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct HeaderExtension {
	pub commitment: KateCommitment,
	pub app_lookup: DataLookup,
}

impl HeaderExtension {
	pub fn data_root(&self) -> H256 {
		self.commitment.data_root.unwrap_or_default()
	}

	pub fn app_lookup(&self) -> &DataLookup {
		&self.app_lookup
	}

	pub fn rows(&self) -> u16 {
		self.commitment.rows
	}

	pub fn cols(&self) -> u16 {
		self.commitment.cols
	}
}
