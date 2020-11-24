FROM rust:alpine 
# AS foobart

RUN  apk add zip git curl && apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing lcov && apk add build-base llvm jq
RUN rustup default nightly && cargo install grcov rustfilt cargo-binutils && rustup component add llvm-tools-preview
WORKDIR /APP
ADD . /APP

RUN git clean -df


# FROM 
#
# COPY --from=
