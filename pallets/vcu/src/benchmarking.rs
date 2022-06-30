// SBP M2 review: Benchmarks are required for Milestone Deliverables
// This module is not included in lib.rs
// Extrinsics' weights are hardcoded
// No weights module (auto-generated from Benchmarks)

// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! VCU pallet benchmarking
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as VCU;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {

    where_clause { where
    T::ProjectId: From<u32>,
    T::VcuId: From<u32>,
    T::AssetId: From<u32>,
}

    force_add_authorized_account {
        let account_id : T::AccountId = account("account_id", 0, 0);
    }: _(RawOrigin::Root, account_id.clone().into())
    verify {
        assert_eq!(
            VCU::<T>::authorized_accounts().len(),
            1
        );
    }

    force_remove_authorized_account {
        let account_id : T::AccountId = account("account_id", 0, 0);
        VCU::<T>::force_add_authorized_account(RawOrigin::Root.into(), account_id.clone().into())?;
    }: _(RawOrigin::Root, account_id.clone().into())
    verify {
        assert_eq!(
            VCU::<T>::authorized_accounts().len(),
            0
        );
    }

    create {
        let account_id : T::AccountId = account("account_id", 0, 0);
        let owner : T::AccountId = account("owner", 0, 1);
        VCU::<T>::force_add_authorized_account(RawOrigin::Root.into(), account_id.clone().into())?;
        let project_id = 1_u32.into();
        let vcu_id = 1_u32.into();
        let vcu_type = VCUType::Single(vcu_id);
        let amount = 100_u32.into();

        let creation_params = VCUCreationParams {
            originator: owner.clone(),
            amount,
            recipient : owner,
            vcu_type: vcu_type.clone(),
        };

    }: _(RawOrigin::Signed(account_id), project_id, creation_params.into())
    verify {
        assert!(
            VCU::<T>::vcus(project_id, vcu_id).is_some()
        );
    }

    mint_into {
        let account_id : T::AccountId = account("account_id", 0, 0);
        let owner : T::AccountId = account("owner", 0, 1);
        VCU::<T>::force_add_authorized_account(RawOrigin::Root.into(), account_id.clone().into())?;
        let project_id = 1_u32.into();
        let vcu_id = 1_u32.into();
        let vcu_type = VCUType::Single(vcu_id);
        let amount = 100_u32.into();

        let creation_params = VCUCreationParams {
            originator: owner.clone(),
            amount,
            recipient : owner.clone(),
            vcu_type: vcu_type,
        };
        VCU::<T>::create(RawOrigin::Signed(account_id.clone()).into(), project_id, creation_params.into())?;
    }: _(RawOrigin::Signed(account_id), project_id, vcu_id, owner, amount)
    verify {
        assert_eq!(
            VCU::<T>::vcus(project_id, vcu_id).unwrap().supply,
            200_u32.into()
        );
    }

    retire {
        let account_id : T::AccountId = account("account_id", 0, 0);
        let owner : T::AccountId = account("owner", 0, 1);
        VCU::<T>::force_add_authorized_account(RawOrigin::Root.into(), account_id.clone().into())?;
        let project_id = 1_u32.into();
        let vcu_id = 1_u32.into();
        let vcu_type = VCUType::Single(vcu_id);
        let amount = 100_u32.into();

        let creation_params = VCUCreationParams {
            originator: owner.clone(),
            amount,
            recipient : owner.clone(),
            vcu_type: vcu_type.clone(),
        };
        VCU::<T>::create(RawOrigin::Signed(account_id).into(), project_id, creation_params.into())?;
    }: _(RawOrigin::Signed(owner), project_id, vcu_id, amount)
    verify {
        assert_eq!(
            VCU::<T>::vcus(project_id, vcu_id).unwrap().supply,
            0_u32.into()
        );
    }

    impl_benchmark_test_suite!(VCU, crate::mock::new_test_ext(), crate::mock::Test);
}
