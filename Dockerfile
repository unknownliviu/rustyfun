FROM rust:1.31

WORKDIR /usr/src/myapp
COPY . .


RUN rustup install nightly
RUN cargo +nightly install racer

RUN echo "foo"
# RUN cargo install --path .


CMD ["target/release/hello_world"]