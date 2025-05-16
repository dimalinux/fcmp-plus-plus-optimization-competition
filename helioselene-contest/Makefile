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
		$(CARGO_NEW) run --release -- $(CURDIR) +1.84.1 | \
			$(CURDIR)/scripts/parse-cycles

.PHONY: fmt
fmt:
	cd $(CONTEST_DIR) && cargo +nightly fmt

.PHONY: lint
lint:
	cd $(CONTEST_DIR) && $(CARGO_NEW) clippy --all-targets

.PHONY: coverage
coverage:
	rm -rf target/coverage
	mkdir -p target/coverage
	CARGO_INCREMENTAL=0 RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE=cargo-runZ-%p-%m.profraw cargo run --release --example profile_ours
	grcov . --binary-path ./target/release/deps/ -s . -t cobertura,html \
		--branch --llvm --ignore-not-existing \
		--ignore '../*' --ignore '/*' -o target/coverage
	rm -f cargo-run*\.profraw
	@printf "\nCoverage is in:\nfile://$(PWD)/target/coverage/html/index.html\n"


# Warning: This reconfigures the kernel to allow profiling for all users and
# programs on the system.
.PHONY: profile
profile:
	sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid'
	sudo sh -c 'echo 0 >/proc/sys/kernel/kptr_restrict'
	RUSTFLAGS="-Cforce-frame-pointers=yes" cargo build --release --examples
	perf record --freq=997 --call-graph dwarf target/release/examples/profile_ours

.PHONY: flamegraph
flamegraph:
	cargo flamegraph --release --example profile_ours

.PHONY: install-deps
install-deps:
	rustup install 1.69.0
	rustup install 1.84.1
	rustup +1.84.1 target add wasm32v1-none
	rustup install nightly # for formatting
	command -v grcov >/dev/null 2>&1 || cargo install grcov
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
	cargo install cargo-show-asm --features disasm
	cargo install flamegraph

.PHONY: asm
asm:
	mkdir -p target/asm
	cargo objdump --example profile_ours --release -- -d -C > target/asm/profile_ours.asm
	cargo asm --disasm --example  profile_ours | grep helioselene
	@echo "\nUse:\n    cargo asm --disasm --example  profile_ours NUMBER"
