p:
	cargo fmt --all

l:
	cargo clippy --all

t:
	cargo test --all

c check:
	cargo check --all

b build:
	cargo build --all

r run:
	cargo run

br build-release:
	cargo build --release --all

w watch:
	fd .rs | entr -s 'clear && make c && make l && make t'