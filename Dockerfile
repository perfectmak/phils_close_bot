FROM rust:latest

WORKDIR /usr/src/phils_close_bot

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/phils_close_bot*

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/phils_close_bot"]

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

#FROM rust:latest as cargo-build
#
#RUN apt-get update
#
#RUN apt-get install musl-tools -y
#
#RUN rustup target add x86_64-unknown-linux-musl
#
#WORKDIR /usr/src/phils_close_bot
#
#COPY Cargo.toml Cargo.toml
#
#RUN mkdir src/
#
#RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
#
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
#
#RUN rm -f target/x86_64-unknown-linux-musl/release/deps/phils_close_bot*
#
#COPY . .
#
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
#
## ------------------------------------------------------------------------------
## Final Stage
## ------------------------------------------------------------------------------
#
#FROM alpine:latest
#
#RUN addgroup -g 1000 phils_close_bot
#
#RUN adduser -D -s /bin/sh -u 1000 -G phils_close_bot phils_close_bot
#
#WORKDIR /home/phils_close_bot/bin/
#
#COPY --from=cargo-build /usr/src/phils_close_bot/target/x86_64-unknown-linux-musl/release/phils_close_bot .
#
#RUN chown phils_close_bot:phils_close_bot phils_close_bot
#
#USER phils_close_bot
#
#CMD ["./phils_close_bot"]