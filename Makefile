# NestGate Makefile
# Quick commands for development and deployment

.PHONY: help test build deploy clean coverage lint fmt check verify

help: ## Show this help message
	@echo "NestGate - Quick Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

test: ## Run all tests
	cargo test --workspace --lib

test-verbose: ## Run tests with output
	cargo test --workspace --lib -- --nocapture

build: ## Build debug version
	cargo build --workspace

build-release: ## Build release version
	cargo build --workspace --release

check: ## Check compilation without building
	cargo check --workspace

lint: ## Run clippy linter
	cargo clippy --all-targets --all-features

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all --check

verify: ## Run deployment readiness check
	./verify_deployment_readiness.sh

deploy-staging: verify build-release ## Deploy to staging
	./QUICK_DEPLOY.sh staging

deploy-production: verify build-release ## Deploy to production (requires confirmation)
	@echo "⚠️  WARNING: Production deployment"
	@echo "Press Ctrl+C to cancel, or Enter to continue..."
	@read
	./QUICK_DEPLOY.sh production

coverage: ## Generate test coverage report
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		cargo tarpaulin --workspace --lib --out Html --output-dir coverage-report; \
		echo "Coverage report: coverage-report/index.html"; \
	else \
		echo "Install tarpaulin: cargo install cargo-tarpaulin"; \
	fi

bench: ## Run benchmarks
	cargo bench --workspace

clean: ## Clean build artifacts
	cargo clean

doc: ## Generate documentation
	cargo doc --workspace --no-deps --open

audit: ## Security audit of dependencies
	@if command -v cargo-audit >/dev/null 2>&1; then \
		cargo audit; \
	else \
		echo "Install cargo-audit: cargo install cargo-audit"; \
	fi

update: ## Update dependencies
	cargo update

outdated: ## Check for outdated dependencies
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		cargo outdated; \
	else \
		echo "Install cargo-outdated: cargo install cargo-outdated"; \
	fi

logs: ## Show Docker logs
	docker-compose logs -f

status: ## Show deployment status
	@echo "=== NestGate Status ==="
	@echo ""
	@echo "Tests:"
	@cargo test --workspace --lib 2>&1 | grep "test result:" || echo "  Run 'make test' to check"
	@echo ""
	@echo "Services:"
	@docker-compose ps 2>/dev/null || echo "  No services running"

all: fmt lint test build ## Run all checks and build

.DEFAULT_GOAL := help

