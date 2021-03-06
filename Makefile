format: ## Check format without changes (alias: fmt)
	cargo fmt --all -- --write-mode=diff
.PHONY: format

fmt: | format
.PHONY: fmt

lint: ## Check code style using clippy
	## if you have clippy as optional dependency, then
	## cargo rustc --features clippy -- -Z no-trans -Z extra-plugins=clippy
	## otherwise, in dev-dependencies
	cargo clippy
.PHONY: lint

test: ## Run unit tests and integration tests
	cargo test
.PHONY: test

coverage: ## Generate coverage report of unit tests using kcov (alias: cov)
	# cargo build --tests
	cargo test --lib --no-run
	#./.tools/check-kcov integration_test
	./.tools/check-kcov lexicon kcov
.PHONY: coverage

cov: | coverage
.PHONY: cov

document: ## Generate documentation files (alias: doc)
	cargo rustdoc -- --document-private-items -Z --display-warnings
.PHONY: document

doc: | document
.PHONY: doc

build: ## Run debug build
	cargo build
.PHONY: build

clean: ## Spruce up
	cargo clean
.PHONY: clean

help: ## Display this message
	@grep -E '^[a-zA-Z_.-]+: ## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ": ## "}; {printf "\033[36m%-11s\033[0m %s\n", $$1, $$2}'
.PHONY: help

.DEFAULT_GOAL = coverage
default: coverage
