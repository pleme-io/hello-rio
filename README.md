# hello-rio

The canonical Rust-authored [caixa](https://github.com/pleme-io/caixa)
`Servico`. Companion to [`programs/hello-world`](https://github.com/pleme-io/programs/tree/main/hello-world)
(the Lisp-authored canonical Servico). Both serve identical JSON, both
render to the same Kubernetes `ComputeUnit` shape — they prove that
caixa is source-language-agnostic for service-class packages.

## What it does

A WASI 0.2 HTTP component, ~50 LoC of Rust on top of [`wstd`](https://crates.io/crates/wstd).
Compiled to `wasm32-wasip2`, runnable directly in `wasmtime serve`.
Three GET routes:

| route | response |
|---|---|
| `/healthz` | `{"status":"ok","served-by":"hello-rio","module-version":"v0.1.0"}` |
| `/` | `{"message":"<GREETING>, <AUDIENCE><PUNCTUATION>","served-by":"hello-rio"}` |
| `/hello` | (alias for `/`) |

Env knobs: `GREETING`, `AUDIENCE`, `PUNCTUATION` (defaults: `Hello`, `rio`, `!`).

## Layout

```
caixa.lisp                            ← (defcaixa :kind Servico …) — typed manifest
servicos/hello-rio.computeunit.yaml   ← K8s ComputeUnit runtime contract
src/main.rs                           ← Rust source — wasm32-wasip2 component
Cargo.toml                            ← wstd + wasi
flake.nix                             ← substrate's wasi-service-flux-flake
```

## Build

```bash
nix build .#wasmModule       # → result/lib/hello-rio.wasm
nix build .#dockerImage      # → wasmtime + .wasm OCI tarball
nix build .#bundle           # → standalone FluxCD HelmRelease bundle
```

## Run

```bash
# Local (wasmtime as the runtime):
wasmtime serve --addr 127.0.0.1:8080 \
  --env GREETING=Hello --env AUDIENCE=world --env PUNCTUATION=! \
  $(nix build .#wasmModule --no-link --print-out-paths)/lib/hello-rio.wasm

curl localhost:8080/hello
# → {"message":"Hello, world!","served-by":"hello-rio"}
```

## Publish

```bash
feira publish              # tag v0.1.0 + push to origin (Zig-style git-tag)
```

## Deploy to rio

```bash
nix run .#render-deploy    # render FluxCD bundle
nix run .#deploy-rio       # render + git commit + push to k8s repo
```

FluxCD reconciles. Once `lareira-tatara-stack` is unsuspended on rio,
the wasm-operator picks up `servicos/hello-rio.computeunit.yaml`
directly and the standalone HelmRelease retires.

## Why "rio"

`rio` is the K3s cluster on a NixOS node in Bristol — the canonical
single-node fleet member. Every fleet hostname follows
`${app}.${cluster}.${location}.${domain}`, so this service lives at
`hello-rio.rio.bristol.quero.cloud` once deployed (the shorter
`hello-rio.quero.cloud` is its public ingress).

## License

MIT — see [LICENSE](./LICENSE).
