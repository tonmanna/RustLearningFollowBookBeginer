FROM rust:1.39
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path ./convertor
WORKDIR /usr/src/app/target/release
CMD ["./convertor_mongooose_graphql"]