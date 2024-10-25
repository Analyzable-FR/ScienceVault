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
    type AccountData = pallet_balances::AccountData<Balance>;
    type AccountId = u64;
    type BaseCallFilter = frame_support::traits::Everything;
    type Block = Block;
    type BlockHashCount = ConstU64<250>;
    type BlockLength = ();
    type BlockWeights = ();
    type DbWeight = ();
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Lookup = IdentityLookup<Self::AccountId>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type MultiBlockMigrator = ();
    type Nonce = u64;
    type OnKilledAccount = ();
    type OnNewAccount = ();
    type OnSetCode = ();
    type PalletInfo = PalletInfo;
    type PostInherents = ();
    type PostTransactions = ();
    type PreInherents = ();
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeTask = ();
    type SS58Prefix = ConstU16<42>;
    type SingleBlockMigrations = ();
    type SystemWeightInfo = ();
    type Version = ();
}

impl pallet_balances::Config for Test {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU32<10>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type RuntimeFreezeReason = ();
    type RuntimeHoldReason = ();
    type WeightInfo = ();
}

impl timestamp::Config for Test {
    type MinimumPeriod = ConstU64<0>;
    type Moment = u64;
    type OnTimestampSet = ();
    type WeightInfo = ();
}

impl pallet_vault::Config for Test {
    type AccountIdOf = ();
    type Currency = ();
    type ElementHash = u8;
    type ElementId = u64;
    type FeePrice = ConstU32<10>;
    type OnFee = ();
    type RewardHandler = ();
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
