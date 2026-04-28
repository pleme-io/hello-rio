//! hello-rio — the canonical Rust-authored caixa Servico.
//!
//! This .wasm is a `wasi:http/proxy` component. It exposes three GET
//! routes that all return JSON:
//!
//!   GET /healthz  — `{"status":"ok","served-by":"hello-rio",…}`
//!   GET /         — `{"message":"<GREETING>, <AUDIENCE><PUNCTUATION>",…}`
//!   GET /hello    — alias for `/`
//!
//! Configuration comes from the env (matching programs/hello-world's
//! shape):
//!
//!   GREETING       default "Hello"
//!   AUDIENCE       default "rio"
//!   PUNCTUATION    default "!"
//!
//! Run locally:
//!
//!   nix build .#wasmModule
//!   wasmtime serve --addr 127.0.0.1:8080 \
//!     --env GREETING=Hello --env AUDIENCE=rio --env PUNCTUATION=! \
//!     result/lib/hello-rio.wasm
//!   curl localhost:8080/hello
//!
//! See caixa.lisp for the typed package manifest and
//! servicos/hello-rio.computeunit.yaml for the cluster-side runtime
//! contract consumed by the wasm-operator.

use wstd::http::body::IncomingBody;
use wstd::http::server::{Finished, Responder};
use wstd::http::{IntoBody, Method, Request, Response, StatusCode};

#[wstd::http_server]
async fn main(request: Request<IncomingBody>, responder: Responder) -> Finished {
    let path = request.uri().path();
    match (request.method(), path) {
        (&Method::GET, "/healthz") => respond_json(responder, StatusCode::OK, healthz_body()).await,
        (&Method::GET, "/" | "/hello") => respond_json(responder, StatusCode::OK, hello_body()).await,
        (&Method::GET, _) => {
            respond_json(responder, StatusCode::NOT_FOUND, r#"{"error":"not found"}"#.to_owned()).await
        }
        _ => {
            respond_json(
                responder,
                StatusCode::METHOD_NOT_ALLOWED,
                r#"{"error":"method not allowed"}"#.to_owned(),
            )
            .await
        }
    }
}

async fn respond_json(responder: Responder, status: StatusCode, body: String) -> Finished {
    let response = Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .header("server", "hello-rio")
        .body(body.into_body())
        .unwrap();
    responder.respond(response).await
}

fn healthz_body() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!(r#"{{"status":"ok","served-by":"hello-rio","module-version":"v{version}"}}"#)
}

fn hello_body() -> String {
    let greeting = std::env::var("GREETING").unwrap_or_else(|_| "Hello".into());
    let audience = std::env::var("AUDIENCE").unwrap_or_else(|_| "rio".into());
    let punctuation = std::env::var("PUNCTUATION").unwrap_or_else(|_| "!".into());

    // Same JSON shape as programs/hello-world for interchangeability.
    format!(r#"{{"message":"{greeting}, {audience}{punctuation}","served-by":"hello-rio"}}"#)
}
