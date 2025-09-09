generate:
	rm -f rust/src/frb_generated.rs &&flutter_rust_bridge_codegen generate

build-ios:
	cd rust && cargo build --release --target aarch64-apple-ios && cd -

run:
	cd example && flutter run && cd -
