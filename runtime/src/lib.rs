#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
pub mod handlers;
pub mod weights;

use frame_support::PalletId;
use frame_system::EnsureRoot;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, One, Verify},
    MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
use frame_support::weights::constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND};
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
        StorageInfo,
    },
    weights::{IdentityFee, Weight},
    StorageValue,
};
pub use frame_system::Call as SystemCall;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub use pallet_reward;
pub use pallet_vault;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("node-template"),
    impl_name: create_runtime_str!("node-template"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 100,
    impl_version: 1,
    apis: apis::RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
            NORMAL_DISPATCH_RATIO,
        );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = frame_support::traits::Everything;
    /// The block type for the runtime.
    type Block = Block;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    type MultiBlockMigrator = ();
    /// The type for storing how many extrinsics an account has signed.
    type Nonce = Nonce;
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// The set code logic, just the default since we're not a parachain.
    type OnSetCode = ();
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    type PostInherents = ();
    type PostTransactions = ();
    type PreInherents = ();
    /// The aggregated dispatch type that is available for extrinsics.
    type RuntimeCall = RuntimeCall;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The ubiquitous origin type.
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeTask = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    type SingleBlockMigrations = ();
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
    /// Version of the runtime.
    type Version = Version;
}

impl pallet_aura::Config for Runtime {
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
}

impl pallet_grandpa::Config for Runtime {
    type EquivocationReportSystem = ();
    type KeyOwnerProof = sp_core::Void;
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<32>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 10_000_000_000_000;

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeEvent = RuntimeEvent;
    type RuntimeFreezeReason = RuntimeHoldReason;
    type RuntimeHoldReason = RuntimeHoldReason;
    type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    type LengthToFee = handlers::LengthToFeeImpl<Balance>;
    type OnChargeTransaction =
        FungibleAdapter<Balances, handlers::HandleDust<TreasuryAccount, Balances>>;
    type OperationalFeeMultiplier = ConstU8<20>;
    type RuntimeEvent = RuntimeEvent;
    type WeightToFee = handlers::WeightToFeeImpl<Balance>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
}

impl pallet_vault::Config for Runtime {
    type AccountIdOf = handlers::AccountIdOf<Runtime>;
    type Currency = Balances;
    type ElementHash = sp_core::H256;
    type ElementId = u64;
    type FeePrice = frame_support::traits::ConstU128<10_000_000_000_000>;
    type OnFee = handlers::HandleDust<TreasuryAccount, Balances>;
    type RewardHandler = handlers::HandleReward<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_vault::WeightInfo<Runtime>;
}

impl pallet_reward::Config for Runtime {
    type Currency = Balances;
    // ~1 minute
    type Dividend = ConstU128<100_000_000_000>;
    type ReevaluationPeriod = ConstU32<10>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_reward::WeightInfo<Runtime>;
}

impl pallet_utility::Config for Runtime {
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(1);
    pub const ProposalBondMinimum: Balance = 1;
    pub const SpendPeriod: BlockNumber = DAYS;
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const MaxApprovals: u32 = 100;
    pub const MaxBalance: Balance = Balance::MAX;
}

parameter_types! {
pub TreasuryAccount: AccountId = Treasury::account_id();
pub const Burn: Permill = Permill::zero();
        }
impl pallet_treasury::Config for Runtime {
    type AssetKind = ();
    type BalanceConverter = frame_support::traits::tokens::UnityAssetBalanceConversion;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
    type Beneficiary = AccountId;
    type BeneficiaryLookup = AccountIdLookup<AccountId, ()>;
    type Burn = Burn;
    type BurnDestination = ();
    type Currency = Balances;
    type MaxApprovals = MaxApprovals;
    type PalletId = TreasuryPalletId;
    type Paymaster = frame_support::traits::tokens::pay::PayFromAccount<Balances, TreasuryAccount>;
    type PayoutPeriod = sp_core::ConstU32<10>;
    type RejectOrigin = EnsureRoot<AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type SpendFunds = ();
    type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
    type SpendPeriod = SpendPeriod;
    type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
    pub struct Runtime {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        Vault: pallet_vault,
        Reward: pallet_reward,
        Utility: pallet_utility,
        Treasury: pallet_treasury,
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckNonZeroSender<Runtime>,
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
    define_benchmarks!(
        [frame_system, SystemBench::<Runtime>]
        [pallet_balances, Balances]
        [pallet_timestamp, Timestamp]
        [pallet_sudo, Sudo]
        [pallet_vault, Vault]
        [pallet_reward, Reward]
        [pallet_utility, Utility]
        [pallet_treasury, Treasury]
    );
}
