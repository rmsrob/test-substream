ENDPOINT ?= mainnet.eth.streamingfast.io:443
START_BLOCK ?= 10762783
STOP_BLOCK ?= +10

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release
	ls -l ./target/wasm32-unknown-unknown/release/substreams.wasm

# .PHONY: stream
# stream: build
# 	substreams run -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

# .PHONY: stream
# package: build
# 	substreams package substreams.yaml