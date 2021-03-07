FROM rustlang/rust:nightly AS build
WORKDIR /app
COPY . /app
RUN cargo update && cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/opa-bundle-proxy /
CMD ["/opa-bundle-proxy"]
