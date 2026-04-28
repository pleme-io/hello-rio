;; hello-rio — the canonical Rust-authored caixa Servico.
;;
;; Companion to programs/hello-world (the Lisp-authored canonical Servico).
;; Both serve the same JSON contract — `{"message":"Hello, …!"}` — proving
;; that caixa is source-language-agnostic for Servicos: a tatara-lisp
;; .tlisp source and a Rust wasm32-wasip2 component are both valid
;; module shapes under one typed manifest.
;;
;; Build:
;;   nix build .#wasmModule           ; → result/lib/hello-rio.wasm
;;   nix build .#dockerImage          ; → wasmtime + .wasm OCI tarball
;;
;; Publish (Zig-style git-tag, the caixa store model):
;;   feira publish                    ; → tags vN.M.P, pushes to origin
;;
;; Cluster dispatch (Phase B, via lareira-tatara-stack + wasm-operator):
;;   the typed Servico manifest at servicos/hello-rio.computeunit.yaml
;;   is rendered by caixa-helm into a ComputeUnit CR; the operator
;;   reconciles it into a Deployment + Service + KEDA ScaledObject.

(defcaixa
  :nome        "hello-rio"
  :versao      "0.1.0"
  :kind        Servico
  :edicao      "2026"
  :descricao   "Canonical Rust→wasm32-wasip2 caixa Servico — complement to programs/hello-world (Lisp). Serves JSON on :8080 / /healthz, ingress hello.quero.cloud."
  :repositorio "github:pleme-io/hello-rio"
  :licenca     "MIT"
  :autores     ("pleme-io")
  :etiquetas   ("hello-world" "wasm" "wasi" "rust" "caixa-servico" "canonical" "breathable")
  :deps        ()
  :deps-dev    ()
  :servicos    ("servicos/hello-rio.computeunit.yaml"))
