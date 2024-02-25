use super::*;
use primitives::{
	CarbonAssetType::{Credits, Donations, Forwards, Shares},
	VerifyCarbonAssetTransfer,
};

impl<T: Config> VerifyCarbonAssetTransfer<T::AccountId, T::AssetId, T::Balance> for Pallet<T> {
	fn is_transfer_allowed(
		sender: T::AccountId,
		recipient: T::AccountId,
		asset_id: T::AssetId,
		amount: T::Balance,
	) -> DispatchResult {
		// check if the asset is a carbon asset
		if let Some((project_id, group_id, asset_type)) = AssetIdLookup::<T>::get(asset_id) {
			// if the recipient is the project owner, no further checks
			let project =
				Projects::<T>::get(project_id).ok_or(Error::<T>::KYCAuthorisationFailed)?;
			if recipient == project.originator {
				return Ok(())
			}

			// fetch the kyc status of the recipient
			if let Some(kyc_level) = T::KYCProvider::get_kyc_level(recipient.clone()) {
				match asset_type {
					Credits | Forwards | Donations => return Ok(()),
					Shares => match kyc_level {
						primitives::UserLevel::KYCLevel4 => return Ok(()),
						_ => return Err(Error::<T>::KYCAuthorisationFailed.into()),
					},
				}
			} else {
				return Err(Error::<T>::KYCAuthorisationFailed.into())
			}
		}

		Ok(())
	}
}
