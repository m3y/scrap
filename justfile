# usage
usage:
	@just -l

# prepare
prepare:
	@cargo fmt --help 2>&1 > /dev/null || rustup component add rustfmt
	@cargo clippy --help 2>&1 > /dev/null || rustup component add clippy


# run
run: prepare
	@RUSTC_WRAPPER={{`which sccache`}} cargo run

# build
build: prepare
	@RUSTC_WRAPPER={{`which sccache`}} cargo build

# fmt
fmt: prepare
	@cargo fmt

# lint
link: prepare
	@cargo clippy

# test
test: prepare
	@RUSTC_WRAPPER={{`which sccache`}} cargo test

# clean
clean:
	@cargo clean

# vim: set noexpandtab :
