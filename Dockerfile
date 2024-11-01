FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=emomtimer
WORKDIR /workspaces/${PROJECT}


USER rust
ENV USER=rust
ENV PATH=/home/${USER}/.cargo/bin:${PATH}::/usr/local/go/bin
# source $HOME/.cargo/env

RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

COPY --chown=rust:rust . .
