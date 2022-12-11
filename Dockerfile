## Build stage
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=rsecho
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /rsecho

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip -s /rsecho/target/x86_64-unknown-linux-musl/release/rsecho

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rsecho

EXPOSE 3000

# Copy our build
COPY --from=builder /rsecho/target/x86_64-unknown-linux-musl/release/rsecho ./

# Use an unprivileged user.
USER rsecho:rsecho

CMD ["/rsecho/rsecho"]