CONTEST_DIR := helioselene-contest-src
CARGO_NEW := cargo +1.84.1
CARGO_OLD := cargo +1.69.0

.PHONY: all
all: fmt build test cycles bench

.PHONY:
build:
	$(CARGO_OLD) build --release
	$(CARGO_NEW) build --tests
	$(CARGO_NEW) build --release
	$(CARGO_NEW) build --release --target wasm32v1-none --no-default-features
	cd helioselene-contest-src && $(CARGO_NEW) build --tests


.PHONY: test
test:
	$(CARGO_OLD) test --release
	$(CARGO_NEW) test # Debug test
	$(CARGO_NEW) test --release
	cd helioselene-contest-src && $(CARGO_NEW) test

.PHONY: bench
bench:
	$(CARGO_NEW) bench

.PHONY: cycles
cycles:
	cd ../../wasm-cycles && \
		$(CARGO_NEW) run --release -- $(CURDIR) +1.84.1

.PHONY: fmt
fmt:
	cd $(CONTEST_DIR) && cargo +nightly fmt

.PHONY: lint
lint:
	cd $(CONTEST_DIR) && $(CARGO_NEW) clippy --all-targets

.PHONY: coverage
coverage:
	command -v grcov >/dev/null 2>&1 || cargo install grcov
	rustup component add llvm-tools-preview
	rm -rf target/coverage
	mkdir -p target/coverage
	CARGO_INCREMENTAL=0 RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE=cargo-test-%p-%m.profraw cargo test
	grcov . --binary-path ./target/debug/deps/ -s . -t cobertura,html --branch --ignore-not-existing --ignore '../*' --ignore '/*' -o target/coverage
	rm -f cargo-test*\.profraw
	@printf "\nCoverage is in:\nfile://$(PWD)/target/coverage/html/index.html\n"

profile:
	RUSTFLAGS="-Cforce-frame-pointers=yes" cargo build --release --examples
	perf record -g --call-graph dwarf target/release/examples/selene_add

flamegraph:
	cargo install flamegraph
	cargo flamegraph --release --examples selene_add
