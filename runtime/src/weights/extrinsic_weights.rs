//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-10 (Y/M/D)
//! HOSTNAME: `bgallois-ms7d43`, CPU: `12th Gen Intel(R) Core(TM) i3-12100F`
//!
//! SHORT-NAME: `extrinsic`, LONG-NAME: `ExtrinsicBase`, RUNTIME: `Development`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `runtime/src/weights/`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1.0`, WEIGHT-ADD: `0`

// Executed Command:
//   target/release/science-vault
//   benchmark
//   overhead
//   --chain=dev
//   --wasm-execution=compiled
//   --weight-path=runtime/src/weights/
//   --warmup=10
//   --repeat=100

use sp_core::parameter_types;
use sp_weights::{constants::WEIGHT_REF_TIME_PER_NANOS, Weight};

parameter_types! {
    /// Time to execute a NO-OP extrinsic, for example `System::remark`.
    /// Calculated by multiplying the *Average* with `1.0` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 71_562, 73_004
    ///   Average:  72_073
    ///   Median:   72_145
    ///   Std-Dev:  310.69
    ///
    /// Percentiles nanoseconds:
    ///   99th: 72_879
    ///   95th: 72_481
    ///   75th: 72_282
    pub const ExtrinsicBaseWeight: Weight =
        Weight::from_parts(WEIGHT_REF_TIME_PER_NANOS.saturating_mul(72_073), 0);
}

#[cfg(test)]
mod test_weights {
    use sp_weights::constants;

    /// Checks that the weight exists and is sane.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn sane() {
        let w = super::ExtrinsicBaseWeight::get();

        // At least 10 µs.
        assert!(
            w.ref_time() >= 10u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
            "Weight should be at least 10 µs."
        );
        // At most 1 ms.
        assert!(
            w.ref_time() <= constants::WEIGHT_REF_TIME_PER_MILLIS,
            "Weight should be at most 1 ms."
        );
    }
}