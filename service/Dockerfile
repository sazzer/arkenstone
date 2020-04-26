# Build the code first
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /arkenstone

RUN apt update -y && apt install -y pkg-config libssl-dev

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY Cargo.lock Cargo.toml docker/pre-build.sh /arkenstone/
RUN ./pre-build.sh
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /arkenstone/target -name "*arkenstone*" | xargs rm -rf

# Finally, we do a full build of our code
RUN rm -rf src
COPY migrations /arkenstone/migrations/
COPY src /arkenstone/src/
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM debian:buster-slim

WORKDIR /arkenstone

RUN apt update -y && apt install -y openssl

COPY --from=builder /arkenstone/target/release/arkenstone /arkenstone/arkenstone

CMD ["/arkenstone/arkenstone"]
