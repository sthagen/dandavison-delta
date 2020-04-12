build:
	cargo build --release

lint:
	cargo clippy

test: unit-test end-to-end-test

unit-test:
	cargo test

end-to-end-test: build
	bash -c "diff -u <(git log -p) <(git log -p | target/release/delta --color-only | perl -pe 's/\e\[[0-9;]*m//g')"

release:
	@make -f release.Makefile release

version:
	@grep version Cargo.toml | head -n1 | sed -E 's,.*version = "([^"]+)",\1,'

hash:
	@version=$$(make version) && \
    printf "$$version-tar.gz %s\n" $$(curl -sL https://github.com/dandavison/delta/archive/$$version.tar.gz | sha256sum -) && \
	printf "delta-$$version-x86_64-apple-darwin.tar.gz %s\n" $$(curl -sL https://github.com/dandavison/delta/releases/download/$$version/delta-$$version-x86_64-apple-darwin.tar.gz | sha256sum -) && \
	printf "delta-$$version-x86_64-unknown-linux-musl.tar.gz %s\n" $$(curl -sL https://github.com/dandavison/delta/releases/download/$$version/delta-$$version-x86_64-unknown-linux-musl.tar.gz | sha256sum -)

BENCHMARK_INPUT_FILE = /tmp/delta-benchmark-input.gitdiff
benchmark: build
	git log -p 23c292d3f25c67082a2ba315a187268be1a9b0ab > $(BENCHMARK_INPUT_FILE)
	hyperfine 'target/release/delta < $(BENCHMARK_INPUT_FILE) > /dev/null'

chronologer:
	chronologer performance/chronologer.yaml

magit-delta-build:
	docker build -f magit/Dockerfile -t dandavison7/magit-delta .

# On MacOS:
# 1. brew cask install xquartz
# 2. XQuartz Settings -> Security -> "Allow connections from network clients"
# 3. Restart XQuartz
magit-delta-run:
	magit/magit-delta-run


.PHONY: build lint test unit-test end-to-end-test release vesion hash benchmark chronologer magit-delta-build magit-delta-run
