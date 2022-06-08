//! Tests for vcu pool pallet
use crate::{mock::*, Config, Error};
use frame_support::{
    assert_noop, assert_ok,
    traits::tokens::fungibles::{metadata::Inspect as MetadataInspect, Inspect},
    PalletId,
};
use frame_system::RawOrigin;
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType};
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::Percent;
use sp_std::convert::TryInto;

pub type VCUPoolEvent = crate::Event<Test>;

#[test]
fn create_new_pools() {
    new_test_ext().execute_with(|| {
        let authorised_account_one = 1;

        assert_ok!(VCUPools::create(
            RawOrigin::Signed(authorised_account_one).into(),
            1,
            Default::default(),
            None,
            "pool_xyz".as_bytes().to_vec().try_into().unwrap(),
        ));

        assert_eq!(
            last_event(),
            VCUPoolEvent::PoolCreated {
                admin: authorised_account_one,
                id: 1,
                config: Default::default()
            }
            .into()
        );
    });
}
