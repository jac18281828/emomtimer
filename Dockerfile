FROM ghcr.io/jac18281828/rustdev:latest

ARG PROJECT=emomtimer
WORKDIR /workspaces/${PROJECT}


USER jac
ENV USER=jac
ENV PATH=/home/${USER}/.cargo/bin:$PATH
# source $HOME/.cargo/env

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

COPY --chown=jac:jac . .

RUN cargo check
RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps
RUN cargo test
RUN trunk build --release

