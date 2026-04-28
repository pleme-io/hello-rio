//! hello-rio — canonical WASI HTTP service deployable to rio.
//!
//! Compiled to wasm32-wasip2 by substrate's wasi-service-flake. Run
//! locally via `nix run .#default`. Deployed to rio's K3s cluster
//! via `nix run .#deploy-rio` (renders + commits the FluxCD bundle
//! into pleme-io/k8s/clusters/rio/services/hello-rio/).
//!
//! For the typed declarative spec consumed by lareira-fleet-programs
//! (Phase B), see the corresponding entry in
//! pleme-io/programs/hello-rio/main.tlisp.

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let greeting = std::env::var("GREETING").unwrap_or_else(|_| "Hello".into());
    let audience = std::env::var("AUDIENCE").unwrap_or_else(|_| "rio".into());
    let punctuation = std::env::var("PUNCTUATION").unwrap_or_else(|_| "!".into());

    eprintln!(
        "hello-rio up — would serve `{greeting}, {audience}{punctuation}` on :{port} \
         (real WASI HTTP listener filled in once wasi-http is stable in this toolchain)."
    );

    // Skeleton — wasi-http binding lands when the toolchain stabilizes.
    // For now this exits cleanly so substrate's wasi-service-flake's
    // wasmtime apps surface (`nix run .#default`) prints the line above
    // and substrate's Docker image starts + reports ready.
}
