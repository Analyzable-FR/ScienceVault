//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-27 (Y/M/D)
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//!
//! DATABASE: `RocksDb`, RUNTIME: `Development`
//! BLOCK-NUM: `BlockId::Number(0)`
//! SKIP-WRITE: `false`, SKIP-READ: `false`, WARMUPS: `1`
//! STATE-VERSION: `V1`, STATE-CACHE-SIZE: ``
//! WEIGHT-PATH: `runtime/src/weights/`
//! METRIC: `Average`, WEIGHT-MUL: `2.0`, WEIGHT-ADD: `0`

// Executed Command:
//   target/release/science-vault
//   benchmark
//   storage
//   --chain=dev
//   --mul=2
//   --weight-path=runtime/src/weights/
//   --state-version=1

/// Storage DB weights for the `Development` runtime and `RocksDb`.
pub mod constants {
    use frame_support::weights::constants;
    use sp_core::parameter_types;
    use sp_weights::RuntimeDbWeight;

    parameter_types! {
        /// By default, Substrate uses `RocksDB`, so this will be the weight used throughout
        /// the runtime.
        pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
            /// Time to read one storage item.
            /// Calculated by multiplying the *Average* of all values with `2.0` and adding `0`.
            ///
            /// Stats nanoseconds:
            ///   Min, Max: 1_309, 809_058
            ///   Average:  25_933
            ///   Median:   1_974
            ///   Std-Dev:  136329.98
            ///
            /// Percentiles nanoseconds:
            ///   99th: 809_058
            ///   95th: 8_763
            ///   75th: 2_425
            read: 51_866 * constants::WEIGHT_REF_TIME_PER_NANOS,

            /// Time to write one storage item.
            /// Calculated by multiplying the *Average* of all values with `2.0` and adding `0`.
            ///
            /// Stats nanoseconds:
            ///   Min, Max: 5_600, 13_617_170
            ///   Average:  411_433
            ///   Median:   9_715
            ///   Std-Dev:  2298831.13
            ///
            /// Percentiles nanoseconds:
            ///   99th: 13_617_170
            ///   95th: 40_062
            ///   75th: 13_655
            write: 822_866 * constants::WEIGHT_REF_TIME_PER_NANOS,
        };
    }

    #[cfg(test)]
    mod test_db_weights {
        use super::constants::RocksDbWeight as W;
        use sp_weights::constants;

        /// Checks that all weights exist and have sane values.
        // NOTE: If this test fails but you are sure that the generated values are fine,
        // you can delete it.
        #[test]
        fn bound() {
            // At least 1 µs.
            assert!(
                W::get().reads(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
                "Read weight should be at least 1 µs."
            );
            assert!(
                W::get().writes(1).ref_time() >= constants::WEIGHT_REF_TIME_PER_MICROS,
                "Write weight should be at least 1 µs."
            );
            // At most 1 ms.
            assert!(
                W::get().reads(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
                "Read weight should be at most 1 ms."
            );
            assert!(
                W::get().writes(1).ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
                "Write weight should be at most 1 ms."
            );
        }
    }
}
