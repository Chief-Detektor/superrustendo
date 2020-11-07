FROM rust:alpine

RUN  apk add zip git && apk add --no-cache -X http://dl-cdn.alpinelinux.org/alpine/edge/testing lcov

WORKDIR /APP
ADD . /APP

RUN git clean -df