//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-10 (Y/M/D)
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
            ///   Min, Max: 1_222, 804_350
            ///   Average:  27_055
            ///   Median:   1_698
            ///   Std-Dev:  139609.27
            ///
            /// Percentiles nanoseconds:
            ///   99th: 804_350
            ///   95th: 6_335
            ///   75th: 2_197
            read: 54_110 * constants::WEIGHT_REF_TIME_PER_NANOS,

            /// Time to write one storage item.
            /// Calculated by multiplying the *Average* of all values with `2.0` and adding `0`.
            ///
            /// Stats nanoseconds:
            ///   Min, Max: 5_853, 12_946_840
            ///   Average:  415_234
            ///   Median:   9_647
            ///   Std-Dev:  2250748.7
            ///
            /// Percentiles nanoseconds:
            ///   99th: 12_946_840
            ///   95th: 34_763
            ///   75th: 12_589
            write: 830_468 * constants::WEIGHT_REF_TIME_PER_NANOS,
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
