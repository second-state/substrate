// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Generic implementation of an extrinsic that has passed the verification
//! stage.

use crate::traits::{
	self, Member, MaybeDisplay, SignedExtension, Dispatchable,
};
use crate::transaction_validity::{TransactionValidity, TransactionSource, InvalidTransaction};

/// Definition of something that the external world might want to say; its
/// existence implies that it has been checked and is good, particularly with
/// regards to the signature.
#[derive(PartialEq, Eq, Clone, sp_core::RuntimeDebug)]
pub struct CheckedExtrinsic<AccountId, Call, Extra> {
	/// Who this purports to be from and the number of extrinsics have come before
	/// from the same signer, if anyone (note this is not a signature).
	pub signed: Option<(AccountId, Extra)>,

	/// The function that should be called.
	pub function: Call,
}

impl<AccountId, Call, Extra, Origin, Info> traits::Applyable for
	CheckedExtrinsic<AccountId, Call, Extra>
where
	AccountId: Member + MaybeDisplay,
	Call: Member + Dispatchable<Origin=Origin>,
	Extra: SignedExtension<AccountId=AccountId, Call=Call, DispatchInfo=Info>,
	Origin: From<Option<AccountId>>,
	Info: Clone,
{
	type DispatchInfo = Info;

	fn validate(
		&self,
		source: TransactionSource,
		info: Self::DispatchInfo,
		len: usize,
	) -> TransactionValidity {
		if let Some((ref id, ref extra)) = self.signed {
			// To maintain backward compatibility, if there is no `SignedExtension`
			// interested in a particular extrinsic we return default value of
			// `ValidTransaction`.
			// Note however that such transaction will not be accepted to the default pool,
			// but if we ever see a block containing the transaction it is going to
			// be accepted (see `pre_dispatch` in `apply`).
			Ok(Extra::validate(extra, id, source, &self.function, info, len)?
				.unwrap_or_default())
		} else {
			Extra::validate_unsigned(source, &self.function, info, len)?
				.ok_or_else(|| InvalidTransaction::NoValidityInfo.into())
		}
	}

	fn apply(
		self,
		info: Self::DispatchInfo,
		len: usize,
	) -> crate::ApplyExtrinsicResult {
		let (maybe_who, pre) = if let Some((id, extra)) = self.signed {
			let pre = Extra::pre_dispatch(extra, &id, &self.function, info.clone(), len)?;
			(Some(id), pre)
		} else {
			let pre = Extra::pre_dispatch_unsigned(&self.function, info.clone(), len)?
				.ok_or(InvalidTransaction::NoValidityInfo)?;
			(None, pre)
		};
		let res = self.function.dispatch(Origin::from(maybe_who));
		Extra::post_dispatch(pre, info.clone(), len);
		Ok(res.map_err(Into::into))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_test_unsigned_logic() {
		assert_eq!(true, false)
	}
}
