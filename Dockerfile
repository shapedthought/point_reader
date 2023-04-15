# stage 1- generata a receipe file for dependencies
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# stage 2- build dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 - use the main officaial rust docker image as our builder
FROM rust as builder

COPY . ./app

# set the working directory to /app
WORKDIR /app

# copy the dependencies file from the cacher stage
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# build the app 
RUN cargo build --release

# stage 4 - use the google distroless image as our final image
FROM ubuntu:latest

# copy the build artifact from the build stage
COPY --from=builder /app/target/release/point_reader /app/point_reader
WORKDIR /app
