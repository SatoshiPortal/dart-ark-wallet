generate:
	rm -f rust/src/frb_generated.rs && flutter_rust_bridge_codegen generate

build-ios:
	cd rust && cargo build --release --target aarch64-apple-ios && cd -

build-android:
	cd rust && cargo ndk -o ../android/app/src/main/jniLibs build

run-example:
	cd example && flutter run && cd -
