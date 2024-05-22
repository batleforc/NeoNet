FROM rust:latest AS builder

WORKDIR /app

# Copy the Cargo.toml and build the project to cache dependencies
COPY Cargo.toml .
RUN mkdir src &&\
    echo "fn main() {}" > src/main.rs && mkdir src/bin &&\
    echo "fn main() {}" > src/bin/sandbox.rs &&\
    echo "fn main() {}" > src/bin/client.rs &&\
    echo "fn main() {}" > src/bin/swag_gen.rs &&\
    echo "fn main() {}" > src/bin/cli.rs
RUN cargo build --release

# Copy the rest of the source code then build the project
COPY src src
RUN touch src/main.rs
RUN cargo build --release

# Strip the binaries to reduce size
RUN strip target/release/server &&\
    strip target/release/client &&\
    strip target/release/swag_gen &&\
    strip target/release/cli

#FROM alpine:latest as release
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

# Copy each binary from the builder stage, all needed for different stages
COPY --from=builder /app/target/release/server .
COPY --from=builder /app/target/release/client .
COPY --from=builder /app/target/release/swag_gen .
COPY --from=builder /app/target/release/cli .

# Copy the resource folder that containt default configuration files
COPY ./resource ./resource

ENV PORT 7777

EXPOSE 7777

CMD ["./server"]