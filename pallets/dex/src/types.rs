use super::*;
use frame_support::{traits::fungibles::Inspect, BoundedVec};
use orml_traits::MultiCurrency;
use primitives::CarbonCreditsValidator;
use sp_runtime::traits::Get;

pub type CurrencyBalanceOf<T> =
	<<T as Config>::Currency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;

pub type AssetBalanceOf<T> =
	<<T as Config>::Asset as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

pub type AssetIdOf<T> =
	<<T as Config>::Asset as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

pub type ProjectIdOf<T> = <<T as Config>::AssetValidator as CarbonCreditsValidator>::ProjectId;

pub type GroupIdOf<T> = <<T as Config>::AssetValidator as CarbonCreditsValidator>::GroupId;

/// ValidatorAccounts type of pallet
pub type ValidatorAccountsListOf<T> =
	BoundedVec<<T as frame_system::Config>::AccountId, <T as pallet::Config>::MaxValidators>;

pub type OrderInfoOf<T> = OrderInfo<
	<T as frame_system::Config>::AccountId,
	AssetIdOf<T>,
	AssetBalanceOf<T>,
	CurrencyBalanceOf<T>,
>;

pub type BuyOrderInfoOf<T> = BuyOrderInfo<
	<T as frame_system::Config>::AccountId,
	AssetIdOf<T>,
	AssetBalanceOf<T>,
	CurrencyBalanceOf<T>,
	<T as frame_system::Config>::BlockNumber,
	<T as Config>::MaxTxHashLen,
	<T as Config>::MaxValidators,
>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct OrderInfo<AccountId, AssetId, AssetBalance, TokenBalance> {
	pub owner: AccountId,
	pub units: AssetBalance,
	pub price_per_unit: TokenBalance,
	pub asset_id: AssetId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct BuyOrderInfo<
	AccountId,
	AssetId,
	AssetBalance,
	TokenBalance,
	Time,
	TxProofLen: Get<u32> + Clone,
	MaxValidators: Get<u32> + Clone,
> {
	pub order_id: OrderId,
	pub buyer: AccountId,
	pub units: AssetBalance,
	pub price_per_unit: TokenBalance,
	pub asset_id: AssetId,
	pub total_fee: TokenBalance,
	pub total_amount: TokenBalance,
	pub expiry_time: Time,
	pub payment_info: Option<PaymentInfo<AccountId, TxProofLen, MaxValidators>>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, MaxEncodedLen, TypeInfo)]
pub struct PaymentInfo<AccountId, TxProofLen: Get<u32> + Clone, MaxValidators: Get<u32> + Clone> {
	pub chain_id: u32,
	pub tx_proof: BoundedVec<u8, TxProofLen>,
	pub validators: BoundedVec<AccountId, MaxValidators>,
}

pub type OrderId = u128;

pub type BuyOrderId = u128;
