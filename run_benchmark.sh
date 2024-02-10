cargo build --release --features runtime-benchmarks
target/release/science-vault benchmark storage --chain=dev --mul=2 --weight-path=runtime/src/weights/ --state-version=1
target/release/science-vault benchmark overhead --chain=dev --wasm-execution=compiled --weight-path=runtime/src/weights/ --warmup=10 --repeat=100
target/release/science-vault benchmark pallet --chain dev --wasm-execution=compiled --pallet "*" --extrinsic "*" --steps 50 --repeat 20 --output runtime/src/weights/
