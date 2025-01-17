# Synkronized

Minimal auto-deployment tool driven by Helm. There's no place like planet homelab!

```yaml
synkronized:
  name: mc-status-rs
  template: single-container
config:
  size: medium
  vaultSecrets:
    - name: "DISCORD_TOKEN"
      path: "mc-status-rs/discord-token"
  env:
    - name: "MAPS_ADDRESS"
      value: "https://maps.vaughn.sh"
    - name: "MC_SERVER_IP"
      value: "mc.vaughn.sh"
```

Snykronized listens to package publish webhooks on repositories, and automatically makes deployments to ArgoCD based on simple, declarative YAML files. 
The latest package version is pulled, and it's injected into a templated helm chart from [synkronized-charts](https://charts.vaughn.sh/). 
Currently, the only injected items are the name and the container image URL on ghcr.io. 
The use of simple helm templates allows for quickly adding new features outside of the Rust API service.

---

## Current Features

- Templating support from synkronized-charts, and pulling the latest chart versions
- Receiving package update webhooks for container deployments to ArgoCD as an application
- Parsing of environment variables and secrets pulled from a local Hashicorp Vault deployment

---

## Planned Features

- Auto provisioned loadbalancers for services that request them
- Auto provisioned ingress and certificates for services that request them
- Support for generic docker containers provided by other projects, as well as docker compose
- Helm chart only repositories that do not use templates
- More robust set of templates
- Multi-container support
- YAML verification for helm values and synkronized.yaml format

---

## Installation and Setup

This project is not ideal to install on any environment other than my own currently. 
I'd like to provide installation instructions and server setup eventually.