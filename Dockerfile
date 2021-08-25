FROM rust:alpine 

RUN  apk add zip git jq build-base
RUN rustup default nightly && cargo install cargo-cov
WORKDIR /APP
ADD . /APP

RUN git clean -df
