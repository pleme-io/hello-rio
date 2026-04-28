# hello-rio — the canonical Rust-authored caixa Servico

> **★★★ CSE / Knowable Construction.** This repo is the canonical
> proof that caixa is source-language-agnostic: a Rust crate and a
> tatara-lisp file are interchangeable shapes for a `:kind Servico`
> caixa, and the same K8s ComputeUnit contract serves both. Companion
> repo: [`pleme-io/programs/hello-world`](https://github.com/pleme-io/programs/tree/main/hello-world)
> (Lisp path). CSE methodology canonical at
> [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md).
> Compounding Directive lives in the org-level `pleme-io/CLAUDE.md` ★★★
> section. End-to-end deploy chain at
> [`pleme-io/theory/WASI-SERVICE-DEPLOY.md`](https://github.com/pleme-io/theory/blob/main/WASI-SERVICE-DEPLOY.md).
> Caixa internals at [`pleme-io/caixa`](https://github.com/pleme-io/caixa).

## What it is

A working WASI 0.2 HTTP component, ~50 LoC of Rust, compiled to
`wasm32-wasip2` and exposed as a typed caixa `Servico`. Three GET routes
returning JSON:

```
GET /healthz   → {"status":"ok","served-by":"hello-rio","module-version":"v0.1.0"}
GET /          → {"message":"Hello, rio!","served-by":"hello-rio"}
GET /hello     → (alias for /)
```

The JSON shape matches `programs/hello-world` exactly so the two
canonical Servicos are interchangeable as smoke tests.

## Layout

This is a typed caixa package per the layout invariants encoded in
`caixa-core::layout::StandardLayout`:

```
caixa.lisp                         ;; (defcaixa :kind Servico …)
servicos/hello-rio.computeunit.yaml  ; the K8s runtime contract
src/main.rs                          ; Rust source — wasm32-wasip2 component
Cargo.toml                           ; wstd + wasi
flake.nix                            ; substrate's wasi-service-flux-flake
LICENSE                              ; MIT
```

The `:servicos` slot in `caixa.lisp` lists the runtime manifest that the
operator + caixa-helm consume. The Rust source under `src/` is treated
as the *implementation* of the Servico — caixa is agnostic about whether
that implementation is Rust→wasm32-wasip2 or tatara-lisp evaluated by
tatara-script. Both flow through the same ComputeUnit contract.

## Build

```bash
nix build .#wasmModule       # → result/lib/hello-rio.wasm (≈ 60–80 KB)
nix build .#dockerImage      # → wasmtime + .wasm OCI tarball
nix build .#bundle           # FluxCD HelmRelease bundle (standalone mode)
nix run   .#default          # Run locally via wasmtime
```

## Run locally without Nix

```bash
cargo build --release --target wasm32-wasip2
wasmtime serve --addr 127.0.0.1:8080 \
  --env GREETING=Hello --env AUDIENCE=world --env PUNCTUATION=! \
  target/wasm32-wasip2/release/hello-rio.wasm

curl localhost:8080/hello
# → {"message":"Hello, world!","served-by":"hello-rio"}
```

## Publish

`feira publish` (from caixa) is the canonical path — it tags HEAD with
`v<versao>` and pushes the tag. Consumers pin `:tag "v0.1.0"` and the
substrate's wasi-service flake builds from that ref.

```bash
feira publish              # tag v0.1.0 + push to origin
```

The OCI image (`ghcr.io/pleme-io/hello-rio:v0.1.0`) is built and pushed
by the release workflow once it lands; until then, build + push manually
or rely on the standalone HelmRelease pulling `:latest` from CI.

## Deploy to rio

Two paths are wired today, one aspirational, one working:

### Standalone (working today)

The flake renders a `bjw-s/app-template` HelmRelease that runs
`wasmtime + /lib/hello-rio.wasm` directly:

```bash
nix run .#render-deploy      # Write bundle to k8s/clusters/rio/services/hello-rio/
nix run .#deploy-rio         # render + git commit + git push (FluxCD reconciles)
```

This works without the wasm-operator or wasm-engine deployed; the Pod
*is* the runtime, wasmtime is the engine.

### lareira (Phase B, aspirational)

Once `lareira-tatara-stack` is unsuspended on rio and the wasm-operator
is publishing properly, the operator picks up the typed manifest at
`servicos/hello-rio.computeunit.yaml` directly. No more `bjw-s/app-template`
indirection — the operator constructs Deployment + Service +
KEDA `ScaledObject` from the ComputeUnit spec.

Flip via `deploy.mode = "lareira"` in `flake.nix` once the operator is
running on rio (re-render, push). The standalone HelmRelease is then
retired in favour of the operator-owned shape.

## Substrate dependencies

- `substrate/lib/wasi-service-flux-flake.nix` — typed deploy-rendering primitive
- `substrate/lib/wasi-service-flake.nix` — wraps wasi-service.nix
- `substrate/lib/wasi-service.nix` — Rust→wasm32-wasip2 build (vendored, component-aware)
- `substrate/lib/module-trio.nix` — HM/Darwin/NixOS module emission

## Why two canonical hello-worlds

`programs/hello-world` is the canonical *Lisp-authored* caixa Servico —
its module is a `.tlisp` file evaluated by tatara-script. `hello-rio` is
the canonical *Rust-authored* caixa Servico — its module is a wasm32-wasip2
component compiled from Rust. Both:

- Declare a `(defcaixa :kind Servico …)` manifest at the package root.
- Expose the same JSON contract on `/`, `/hello`, `/healthz`.
- Render to the same K8s ComputeUnit shape.
- Are interchangeable as cluster smoke tests.

The two together prove that caixa's typed surface is the *only* contract
operators care about — implementation language is a bottom-edge detail
the substrate hides behind one renderer per module kind.
