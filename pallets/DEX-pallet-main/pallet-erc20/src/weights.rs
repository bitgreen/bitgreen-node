use frame_support::weights::Weight;

pub trait WeightInfo {
	fn transfer() -> Weight;

	fn approve() -> Weight;

	fn increase_allowance() -> Weight;

	fn decrease_allowance() -> Weight;

	fn transfer_from() -> Weight;

	fn mint() -> Weight;

	fn burn() -> Weight;
}

impl WeightInfo for () {
	fn transfer() -> Weight {
		10_000
	}

	fn approve() -> Weight {
		10_000
	}

	fn increase_allowance() -> Weight {
		10_000
	}

	fn decrease_allowance() -> Weight {
		10_000
	}

	fn transfer_from() -> Weight {
		10_000
	}

	fn mint() -> Weight {
		10_000
	}

	fn burn() -> Weight {
		10_000
	}
}
