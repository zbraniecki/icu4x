# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

ICU4X_NIGHTLY_TOOLCHAIN ?= "nightly-2022-12-26"

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
WASM_STACK_SIZE := 100000
 
lib/index.js:
	rm -rf lib
	cp -r ../capi/bindings/js lib

target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-args=-zstack-size=${WASM_STACK_SIZE} -Clto -Cembed-bitcode" \
	CARGO_TARGET_DIR="target" \
	cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
		-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
		--target wasm32-unknown-unknown \
		--release \
		-p icu_capi_cdylib \
		--features buffer_provider

build: lib/index.js target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm