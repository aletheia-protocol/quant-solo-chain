use crate as pallet_pow;
use frame_support::{parameter_types, traits::Everything};
use sp_core::H256;
use sp_runtime::{testing::{Header}, traits::{BlakeTwo256, IdentityLookup}, BuildStorage};


pub type TestHeader = Header;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

pub type Block = sp_runtime::generic::Block<TestHeader, UncheckedExtrinsic>;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        PoW: pallet_pow,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32::<16>;
	type Nonce = u64;
	type RuntimeTask = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
	type RuntimeCall = RuntimeCall;
	type Block = Block;
}

impl pallet_pow::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_pow::DefaultWeightInfo;
}

// Build the test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	sp_io::TestExternalities::new(t)
}