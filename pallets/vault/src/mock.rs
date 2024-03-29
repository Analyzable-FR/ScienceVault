use crate as pallet_vault;
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use pallet_timestamp::{self as timestamp};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u32;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Timestamp: timestamp,
        VaultModule: pallet_vault,
        Balances: pallet_balances,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type PostInherents = ();
    type PostTransactions = ();
    type PreInherents = ();
    type MultiBlockMigrator = ();
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU32<10>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
}

impl timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<0>;
    type WeightInfo = ();
}

impl pallet_vault::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type ElementId = u64;
    type ElementHash = u8;
    type RewardHandler = ();
    type AccountIdOf = ();
    type Currency = ();
    type FeePrice = ConstU32<10>;
    type OnFee = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
