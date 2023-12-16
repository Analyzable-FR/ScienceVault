CC=gcc-12 CXX=g++-12 cargo build --release --features runtime-benchmarks
target/release/science-vault benchmark pallet --chain dev --wasm-execution=compiled --pallet "*" --extrinsic "*" --steps 50 --repeat 20 --output runtime/src/weights/
