![WIP](https://img.shields.io/badge/status-wip-red)

[![Latest release](https://img.shields.io/github/v/release/ConcaveFi/substream-concave)](https://github.com/ConcaveFi/substream-concave/releases/latest)
<a href="https://gitpod.io/#https://github.com/ConcaveFi/substream-concave" target="_blank">
  <img
    src="https://img.shields.io/badge/Open%20with-Gitpod-908a85?logo=gitpod"
    alt="Contribute with Gitpod"
  />
</a>

# substream-concave

> Tracking data for concave smart contracts

## Install
### Prerequisite
> **Note**
>
> Get a [substreams API KEY](https://substreams.streamingfast.io/reference-and-specs/authentication#obtain-your-api-key) and past it in the `.env`, manage also other entries in regards to the smart contracts you're working on.
> 
> If you run it with Gitpod make sure to add the `STREAMINGFAST_KEY` inside your account page environements.
>Required:
> - [Rust](https://rust-lang.github.io/rustup/installation/index.html)
> - [substream CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
> - [buf](https://docs.buf.build/installation)

```bash
git clone git@github.com:ConcaveFi/substream-concave.git
# copie the .env.example to .env
```

### Generating Protobuf

- This will first build a `.rs` file inside the `src/abis`. The name of this file will be the same as the `abis/*.json`.
- Then, it will read the package name of your `*.proto` files inside `proto`, and create a file with the same name, with all the rust types inside the `src/pb` directory **IF** you have declare it in side the `substreams.yaml` under the `protobuf:` section.
- Will also create the `pb/*.rs` file from declare `imports` `*.spkg` file in the `substreams.yaml`.

```bash
make protogen
# substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

### Build

- Check the names consistency inside the `substreams.yaml` `package: name:` it should be the same across other files:
  - `substreams.yaml` binaries file `*.wasm`
  - `Cargo.toml` package name
- The build will be under `./target/wasm32-unknown-unknown/release/*.wasm`

```bash
make build
# cargo build --target wasm32-unknown-unknown --release
```

### WIP !Run Substream

- Get the `src/lib.rs` map function name and pass it to the run cli.
- Check the `.env` for the blocks info
- The cmd `make run` will run all prior cmds and will refresh & export `SUBSTREAMS_API_TOKEN=${SF_TOKEN}`
- Make sure the names are reported inside the `substreams.yaml` modules.
- Da Fook z'gooinnn on!

```bash
make run
# substreams run -e $(ENDPOINT) substreams.yaml map_stakings -s $(START_BLOCK) -t $(STOP_BLOCK)
```

### WIP Pack spkg

- write the substream protogen substreams.spkg in root directory

```bash
make package
# substreams pack -o substreams.spkg substreams.yaml
```