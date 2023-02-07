![WIP](https://img.shields.io/badge/status-wip-red)

[![Latest release](https://img.shields.io/github/v/release/ConcaveFi/substream-concave)](https://github.com/ConcaveFi/substream-concave/releases/latest)
<a href="https://gitpod.io/#https://github.com/ConcaveFi/substream-concave" target="_blank">
  <img
    src="https://img.shields.io/badge/Open%20with-Gitpod-908a85?logo=gitpod"
    alt="Contribute with Gitpod"
  />
</a>

# Substream-Concave

> Tracking data for concave smart contracts

## Install
### Prerequisite
> **Note**
>
>Required:
- [Rust](https://rust-lang.github.io/rustup/installation/index.html)
- [substream CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
- [buf](https://docs.buf.build/installation)

```bash
git clone git@github.com:ConcaveFi/substream-concave.git
cd substream-concave
cp .env.example .env
```

Get a [substreams API KEY](https://substreams.streamingfast.io/reference-and-specs/authentication#obtain-your-api-key) and past it in the `.env`

### Generating Protobuf
```bash
make protogen
# substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

### Build
```bash
make build
# cargo build --target wasm32-unknown-unknown --release
```

## Run Substream
WIP
