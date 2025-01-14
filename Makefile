release:
	cargo build --release

test:
	cargo test

run:
	cargo run

wasm:
	wasm-pack build --release --target web --out-dir server/pkg

publish:
	cargo test
	cargo publish

docker:
	docker build ./server -t git.betalupi.com/mark/daisy
