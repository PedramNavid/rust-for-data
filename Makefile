.PHONY: all build build-release serve benchmarks
build-release:
	cargo build --release --manifest-path=./wxrs/Cargo.toml --target-dir=./wxrs/target

serve:
	cd rust4data-book && make $@

benchmarks:
	cd benchmarks && make all
