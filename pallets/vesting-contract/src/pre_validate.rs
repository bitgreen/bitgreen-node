// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//
#![allow(clippy::all)]
use codec::{Decode, Encode};
use frame_support::{
	dispatch::fmt::Debug,
	pallet_prelude::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
	traits::IsSubType,
};
use scale_info::TypeInfo;
use sp_runtime::traits::{DispatchInfoOf, SignedExtension};

use super::*;
use crate::{Call, Config};

/// Validate `attest` calls prior to execution. Needed to avoid a DoS attack since they are
/// otherwise free to place on chain.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PrevalidateVestingWithdraw<T: Config + Send + Sync>(sp_std::marker::PhantomData<T>)
where
	<T as frame_system::Config>::Call: IsSubType<Call<T>>;

impl<T: Config + Send + Sync> Debug for PrevalidateVestingWithdraw<T>
where
	<T as frame_system::Config>::Call: IsSubType<Call<T>>,
{
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "PrevalidateVestingWithdraw")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> PrevalidateVestingWithdraw<T>
where
	<T as frame_system::Config>::Call: IsSubType<Call<T>>,
{
	/// Create new `SignedExtension` to check runtime version.
	pub fn new() -> Self {
		Self(sp_std::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> SignedExtension for PrevalidateVestingWithdraw<T>
where
	<T as frame_system::Config>::Call: IsSubType<Call<T>>,
{
	type AccountId = T::AccountId;
	type AdditionalSigned = ();
	type Call = <T as frame_system::Config>::Call;
	type Pre = ();

	const IDENTIFIER: &'static str = "PrevalidateVestingWithdraw";

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		Ok(())
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		Ok(self.validate(who, call, info, len).map(|_| ())?)
	}

	// <weight>
	// The weight of this logic is included in the `attest` dispatchable.
	// </weight>
	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		if let Some(local_call) = call.is_sub_type() {
			if let Call::withdraw_vested {} = local_call {
				let _ = VestingContracts::<T>::get(who).ok_or_else(|| {
					InvalidTransaction::Custom(ValidityError::SignerHasNoContract.into())
				})?;
			}
		}
		Ok(ValidTransaction::default())
	}
}

/// Custom validity errors for pallet
#[repr(u8)]
pub enum ValidityError {
	/// The signer has no contract.
	SignerHasNoContract = 0,
}

impl From<ValidityError> for u8 {
	fn from(err: ValidityError) -> Self {
		err as u8
	}
}
