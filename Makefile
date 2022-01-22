.PHONY: all clean format build run

all: build

clean:
	@cargo clean

format:
	@cargo +nightly fmt -- --emit=files
	@cargo clippy -- --no-deps

build:
	@cargo build --release

run: build
	@cargo run
