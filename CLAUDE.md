# hello-rio — canonical WASI HTTP service deployable to rio

> **★★★ CSE / Knowable Construction.** This repo is the canonical
> proof of substrate's `wasi-service-flux-flake` — it takes a typed
> `module = { ... }` + `deploy = { ... }` spec and produces every
> artifact rio's K3s cluster needs to schedule the service. CSE
> methodology canonical at
> [`pleme-io/theory/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md`](https://github.com/pleme-io/theory/blob/main/CONSTRUCTIVE-SUBSTRATE-ENGINEERING.md).
> The Compounding Directive is in the org-level pleme-io/CLAUDE.md ★★★
> section. End-to-end deploy chain documented at
> [`pleme-io/theory/WASI-SERVICE-DEPLOY.md`](https://github.com/pleme-io/theory/blob/main/WASI-SERVICE-DEPLOY.md).

## What it does

A minimal WASI HTTP service. Compiled to `wasm32-wasip2`, packaged as
a Docker image (`ghcr.io/pleme-io/hello-rio:<tag>`), wrapped in a
FluxCD HelmRelease, deployed to `rio`'s `tatara-system` namespace, and
exposed at `hello.quero.cloud` via Cloudflare Tunnel ingress.

Reads four env vars (`PORT`, `GREETING`, `AUDIENCE`, `PUNCTUATION`)
and serves `<greeting>, <audience><punctuation>` on `/`.

## Build

```bash
nix build .#default          # Docker image (substrate's wasi-service-flake)
nix run .#default            # Run locally via wasmtime
nix build .#bundle           # FluxCD deploy bundle (HelmRelease + kustomize)
```

## Deploy to rio

```bash
nix run .#render-deploy      # Write bundle to k8s/clusters/rio/services/hello-rio/
nix run .#deploy-rio         # render + git commit + git push (FluxCD reconciles)
```

The image must be pushed to GHCR separately (`docker push ghcr.io/pleme-io/hello-rio:<tag>`)
— substrate's wasi-service-flake doesn't push automatically. Use the
`forge push` workflow at substrate/.github/workflows/image-push.yml or
push manually for now.

## Substrate dependencies

- `substrate/lib/wasi-service-flux-flake.nix` (the load-bearing primitive)
- `substrate/lib/wasi-service-flake.nix` (wraps wasi-service.nix; wasm + Docker image)
- `substrate/lib/module-trio.nix` (HM/Darwin/NixOS module emission)

## Migration path: standalone → lareira

The current `flake.nix` uses `mode = "standalone"` — emits a stand-alone
HelmRelease backed by `bjw-s/app-template`. Works today without the
`wasm-operator` or `wasm-engine` images deployed.

When `lareira-tatara-stack` is unsuspended in `pleme-io/k8s` (after
Phase B publishes `ghcr.io/pleme-io/wasm-{operator,engine}:0.1.0`):

1. Flip `deploy.mode = "lareira"` in this flake.
2. Re-render: `nix run .#render-deploy`.
3. The bundle now contributes a `programs:` entry to
   `lareira-fleet-programs` instead of a stand-alone HelmRelease.
4. Drop the per-service kustomize directory; lareira-fleet-programs
   aggregates everything.
