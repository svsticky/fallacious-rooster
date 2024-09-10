all: ci

.PHONY: help
help:
	@echo "Fallacious Rooster Makefile Help"
	@echo "- ci-frontend		Run CI only for the frontend"
	@echo "- ci-server		Run CI only for the server"
	@echo "- ci			Run all CI jobs"
	@echo "- server-fmt-fix	Apply fixes to formatting of the server"

.PHONY: ci
ci: ci-frontend ci-server

.PHONY: ci-frontend
ci-frontend: frontend-eslint

.PHONY: ci-server
ci-server: server-fmt server-clippy server-test

.PHONY: server-fmt
server-fmt: install-rustfmt
	cd server && \
		cargo fmt --all --check

.PHONY: server-fmt-fix
server-fmt-fix: install-rustfmt
	cd server && \
		cargo fmt --all

.PHONY: server-clippy
server-clippy: install-clippy
	cd server && \
		cargo clippy

.PHONY: server-test
server-test: install-rust
	cd server && \
		cargo test

.PHONY: frontend-eslint
frontend-eslint: install-frontend-modules
	cd frontend && \
		yarn eslint

.PHONY: install-rustfmt
install-rustfmt: install-rust
	rustup component add rustfmt

.PHONY: install-clippy
install-clippy: install-rust
	rustup component add clippy

.PHONY: install-frontend-modules
install-frontend-modules:
	cd frontend && \
		yarn

.PHONY: install-rust
install-rust:
	rustup toolchain install stable --profile minimal
