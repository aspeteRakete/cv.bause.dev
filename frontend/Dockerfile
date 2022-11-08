FROM rust:1.64 as builder
# -1
RUN rustup target add wasm32-unknown-unknown

# 0. install trunk
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli

# 1. Create a new empty shell project
RUN USER=root cargo new --bin frontend
WORKDIR /frontend

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./index.html ./index.html
COPY ./index.scss ./index.scss
COPY ./bootstrap-5.2.2/scss ./bootstrap-5.2.2/scss
COPY ./img ./img
COPY ./src ./src


# build for release
RUN trunk build --release

# our final base
FROM joseluisq/static-web-server:2

# copy the build artifact from the build stage
COPY --from=builder /frontend/dist /dist

# set the startup command to run your binary
CMD ["--root", "/dist"]


