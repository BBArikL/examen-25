FROM rust/slim-bullseye

RUN cargo install cargo-binutils

RUN rustup component add llvm-tools

ADD ci-cd .

ENTRYPOINT [ "bash" ]