ifneq (,$(wildcard ./.env))
    include .env
    export
endif

cmd-exists-%:
	@hash $(*) > /dev/null 2>&1 || \
		(echo "ERROR: '$(*)' must be installed and available on your PATH."; exit 1)

SF_TOKEN := $(shell curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'${SF_APIKEY}'"}' | jq -r .token)
export SUBSTREAMS_API_TOKEN=${SF_TOKEN}
export FIREHOSE_API_TOKEN=${SF_TOKEN}


.PHONY: gettoken
gettoken: cmd-exists-substreams
	@echo "Token set on FIREHOSE & SUBSTREAMS_API_TOKEN ${SF_TOKEN}\n\n"
	cargo check

.PHONY: protogen
protogen: gettoken
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release
	@ echo "CHECKING RELEASE"
	@ls -l ./target/wasm32-unknown-unknown/release/substreams_concave.wasm
	@ echo "\n"

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml map_stakings -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: package
package: build
	substreams pack -o substreams.spkg substreams.yaml