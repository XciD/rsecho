## Build stage
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /rsecho

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip -s /rsecho/target/x86_64-unknown-linux-musl/release/rsecho

####################################################################################################
## Final image
####################################################################################################
FROM ubuntu

WORKDIR /rsecho
EXPOSE 3000

# Copy our build
COPY --from=builder /rsecho/target/x86_64-unknown-linux-musl/release/rsecho ./

CMD ["/rsecho/rsecho"]
