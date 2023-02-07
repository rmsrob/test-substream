ifneq (,$(wildcard ./.env))
    include .env
    export
endif


MY_TOKEN := $(shell curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'${SF_APIKEY}'"}' | jq -r .token)
export SUBSTREAMS_API_TOKEN=${MY_TOKEN}
export FIREHOSE_API_TOKEN=${MY_TOKEN}


.PHONY: gettoken
gettoken:
	@echo "Token set on FIREHOSE & SUBSTREAMS_API_TOKEN ${MY_TOKEN}\n\n"

.PHONY: protogen
protogen: gettoken
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release
	ls -l ./target/wasm32-unknown-unknown/release/substreams_concave.wasm

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_stakings -s $(START_BLOCK) -t $(STOP_BLOCK)

# .PHONY: stream
# package: build
# 	substreams package substreams.yaml