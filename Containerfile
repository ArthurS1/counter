FROM rust:alpine

# needed dependency
RUN apk add musl-dev

WORKDIR /app

# copy a dummy main that we'll use only to cache dependencies
COPY ./src/dummy.rs /app

# copy rust dependency manifests
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock

# replace main.rs by dummy.rs as main file in the cargo.toml manifest
RUN sed -i 's/src\/main.rs/dummy.rs/g' Cargo.toml

# build and cache dependencies
RUN cargo build #--release

# reverses step 5
RUN sed -i 's/dummy.rs/src\/main.rs/g' Cargo.toml

# copy other files
COPY . /app

# build source code (dependencies already cached)
RUN cargo build --offline #--release

EXPOSE 8080

CMD ["cargo", "run"]
