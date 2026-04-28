{
  description = "hello-rio — canonical WASI HTTP service deployable to rio via substrate's wasi-service-flux-flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
    substrate = {
      url = "github:pleme-io/substrate";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, substrate, fenix, ... }:
    (import "${substrate}/lib/wasi-service-flux-flake.nix" {
      inherit nixpkgs substrate fenix;
    }) {
      inherit self;
      serviceName = "hello-rio";
      wasiCapabilities = [ "network" "env" ];

      module = {
        description = "hello-rio — canonical WASI HTTP service";
        hmNamespace = "blackmatter.components";
      };

      # Deploy to rio's K3s cluster via FluxCD.
      # `nix run .#render-deploy` writes the bundle into k8s/clusters/rio/services/hello-rio/
      # `nix run .#deploy-rio` does that + git commit + git push.
      deploy = {
        cluster = "rio";
        namespace = "tatara-system";
        imageRepo = "ghcr.io/pleme-io";
        imageTag = "latest";
        port = 8080;
        healthPath = "/healthz";
        config = {
          PORT = "8080";
          GREETING = "Hello";
          AUDIENCE = "rio";
          PUNCTUATION = "!";
        };
        ingress = {
          enabled = true;
          host = "hello.quero.cloud";
          className = "nginx";
          path = "/";
        };
        # Standalone HelmRelease (bjw-s/app-template) — works today
        # without lareira-tatara-stack. Switch to mode = "lareira"
        # once the operator + engine images are published and the
        # tatara-stack HelmRelease unsuspended.
        mode = "standalone";
        breathability = {
          enabled = true;
          minReplicas = 1;     # MVP: always 1 (no KEDA HTTP add-on assumed)
          maxReplicas = 3;
          cooldownPeriod = 600;
        };
      };
    };
}
