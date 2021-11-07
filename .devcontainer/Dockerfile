# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.134.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

USER vscode

RUN rustup target add wasm32-unknown-unknown

ADD wasm-pack /usr/local/cargo/bin/wasm-pack

RUN cargo install \
    wasm-bindgen-cli \
    wasmtime-cli \
    simple-http-server

USER root

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
