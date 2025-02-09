
NVA_API_URL ?= "https://api.navability.io/graphql"
NVA_API_TOKEN ?= ""

clean:
	cargo clean
	rm -rf test/build

test-tokio:
	cargo test -F tokio

build-tokio:
	cargo build -F tokio

build-wasm:
	cargo build -F wasm

fetch-schema:
	@graphql-client introspect-schema --authorization $(NVA_API_TOKEN) --output src/schema.json $(NVA_API_URL)

# duplicate of cargo.toml::build-dependencies
install-deps:
	cargo install graphql_client_cli

