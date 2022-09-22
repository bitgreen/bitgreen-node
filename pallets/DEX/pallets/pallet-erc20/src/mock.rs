use crate as pallet_erc20;
use crate::{AfterTransfer, BeforeTransfer, U256};
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Erc20Token: pallet_erc20::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}
type AccountId = u64;

parameter_types! {
    pub const Decimals: u8 = 18;
    pub Name: Vec<u8> = b"Token".to_vec();
    pub Symbol: Vec<u8> = b"TOK".to_vec();
}

impl BeforeTransfer<AccountId> for () {
    fn before_transfer(_from: Option<&AccountId>, _to: &AccountId, _amount: U256) {}
}

impl AfterTransfer<AccountId> for () {
    fn after_transfer(_from: Option<&AccountId>, _to: &AccountId, _amount: U256) {}
}

impl pallet_erc20::Config for Test {
    type Event = Event;
    type Name = Name;
    type Symbol = Symbol;
    type Decimals = Decimals;
    type BeforeTransfer = ();
    type AfterTransfer = ();
    type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
