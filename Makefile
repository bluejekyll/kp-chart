TARGET_DIR ?= ./target
CARGO_TARGET_DIR := ${TARGET_DIR}
MODE ?= development
RUSTFLAGS := -Ctarget-cpu=generic
TMP_DIST_DIR := /tmp/kp-chart-dist

.PHONY: init
init:
	@echo "========> $@"
	@rustup --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh)
	rustup self update
	rustup update
	rustup target add wasm32-unknown-unknown
	cargo install wasm-pack
	npm --version
	npm install

.PHONY: build
wasm:
	@echo "========> $@"
	wasm-pack build

.PHONY: build
build:
	@echo "========> $@"
	npm run build -- --mode=${MODE}

.PHONY: test
test: build
	@echo "========> $@"
	cargo test
	npm run test

.PHONY: run
run: build
	@echo "========> $@"
	npm run start

.PHONY: clean
clean:
	@echo "========> $@"
	rm -rf ./pkg
	rm -rf ./dist


.PHONY: deploy
deploy: clean
	@echo "========> $@"
	@git --version

	# build the project
	$(MAKE) MODE=production WASM_MODE=--release build

	# deploy
	git worktree add ${TMP_DIST_DIR} gh-pages
	rm -rf ${TMP_DIST_DIR}/*
	cp -rp dist/* ${TMP_DIST_DIR}
	cd ${TMP_DIST_DIR} && \
		git add -A && \
		git diff --staged --quiet || \
			(git commit -m "deployed on $(shell date) by ${USER}" && \
				git push origin gh-pages)
	$(MAKE) clean_worktree

.PHONY: clean_worktree
clean_worktree:
	@echo "========> $@"
	rm -rf ${TMP_DIST_DIR}
	git worktree prune
