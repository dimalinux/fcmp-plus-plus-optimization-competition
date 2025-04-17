CONTEST_DIR := helioselene-contest-src
CARGO_NEW := cargo +1.84.1
CARGO_OLD := cargo +1.69.0

.PHONY: all
all: build fmt test bench cycles

.PHONY:
build:
	$(CARGO_OLD) build --release
	$(CARGO_NEW) build
	$(CARGO_NEW) build --release


.PHONY: test
test:
	$(CARGO_NEW) test
	$(CARGO_NEW) test --release
	$(CARGO_OLD) test --release

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
