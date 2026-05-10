# Superrustendo Makefile
# Common development tasks

.PHONY: help build check test fmt clippy doc clean run dis disdbg

# Default: show help
help:
	@echo "Superrustendo Development Tasks"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build          - Build without GUI (default)"
	@echo "  make build-gui      - Build with GUI support"
	@echo "  make check          - Run cargo check"
	@echo "  make check-gui      - Run cargo check with GUI"
	@echo ""
	@echo "Test Commands:"
	@echo "  make test           - Run all tests"
	@echo "  make test-gui       - Run tests with GUI"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt            - Format all code"
	@echo "  make clippy         - Run clippy linter"
	@echo "  make doc            - Generate documentation"
	@echo ""
	@echo "Run Commands:"
	@echo "  make run ROM=file   - Run emulator with ROM"
	@echo "  make dis ROM=file   - Run disassembler"
	@echo "  make disdbg ROM=file - Run debugger"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean          - Clean build artifacts"

# Build without GUI (default)
build:
	cargo build --no-default-features

# Build with GUI
build-gui:
	cargo build --features gui

# Check without GUI
check:
	cargo check --no-default-features

# Check with GUI
check-gui:
	cargo check --features gui

# Run tests without GUI
test:
	cargo test --no-default-features

# Run tests with GUI
test-gui:
	cargo test --features gui

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy --no-default-features -- -D warnings

# Generate documentation
doc:
	cargo doc --no-default-features --open

# Clean
clean:
	cargo clean

# Run emulator
run:
	@if [ -z "$(ROM)" ]; then \
		echo "Error: Please specify ROM file with ROM=path/to/file.sfc"; \
		exit 1; \
	fi; \
	cargo run --no-default-features -- $${ROM}

# Run disassembler
dis:
	@if [ -z "$(ROM)" ]; then \
		echo "Error: Please specify ROM file with ROM=path/to/file.sfc"; \
		exit 1; \
	fi; \
	cargo run --no-default-features --bin snesdis $${ROM}

# Run debugger
disdbg:
	@if [ -z "$(ROM)" ]; then \
		echo "Error: Please specify ROM file with ROM=path/to/file.sfc"; \
		exit 1; \
	fi; \
	cargo run --no-default-features --bin snesdbg $${ROM}
