#!/usr/bin/env bash


# export CARGO_INCREMENTAL=0
# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"
# cargo test --verbose $CARGO_OPTIONS

# zip -0 ccov.zip "$(find . \( -name "superrustendo*.gc*" \) -print)";
# ./grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" -o lcov.info;


# rustup component add llvm-tools-preview
# export RUSTFLAGS="-Zinstrument-coverage"
# cargo build --verbose
# # LLVM_PROFILE_FILE="target/cov/foobart-%p-%m.profraw" 
# cargo test --verbose
# llvm-profdata merge -sparse default.profraw -o default.profdata
# grcov . --binary-path ./target/debug -s . -t lcov --branch --ignore-not-existing  -o lcov.info
# # bash <(curl -s https://codecov.io/bash) -f lcov.info

# genhtml -o ./target/cov/ --show-details --highlight --ignore-errors source --legend lcov.info

# #genhtml target/debug/lcov.info -o target/cov


# curl -L https://github.com/mozilla/grcov/releases/latest/download/grcov-linux-x86_64.tar.bz2 | tar jxf -
# export RUSTFLAGS="-Zinstrument-coverage"
# cargo build --verbose
# LLVM_PROFILE_FILE="your_name-%p-%m.profraw" cargo test --verbose
# ./grcov . --binary-path ./target/debug/superrustendo -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
# # bash <(curl -s https://codecov.io/bash) -f lcov.info

# genhtml -o ./target/cov/ --show-details --highlight --ignore-errors source --legend lcov.info


rustup component add llvm-tools-preview
export RUSTFLAGS="-Zinstrument-coverage"
# cargo build --bin superrustendo --verbose
cargo build --verbose
RUSTFLAGS="-Zinstrument-coverage" \
RUSTDOCFLAGS="-Zinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins" \
LLVM_PROFILE_FILE="target/debug/superrustendo-%p-%m.profraw" cargo test --verbose
llvm-profdata merge -sparse target/debug/superrustendo*.profraw -o target/debug/superrustendo.profdata

cargo cov -- report \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-Zinstrument-coverage" \
          RUSTDOCFLAGS="-Zinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins" \
            cargo test --no-run --message-format=json \
              | jq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ) \
        target/debug/doctestbins/*/rust_out; \
      do \
        [[ -x $file ]] && printf "%s %s " -object $file; \
      done \
    ) \
  --use-color --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='tests/' --ignore-filename-regex='/usr/local/cargo' \
  --instr-profile=target/debug/superrustendo.profdata --summary-only \
  | grep -E '^TOTAL' | grep '[[:alnum:]]*\.[[:alnum:]]*%' -o | head -n1 | xargs echo Coverage

cargo cov -- show \
     $( \
      for file in \
        $( \
          RUSTFLAGS="-Zinstrument-coverage" \
          RUSTDOCFLAGS="-Zinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins" \
            cargo test --no-run --message-format=json \
              | jq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ) \
        target/debug/doctestbins/*/rust_out; \
      do \
        [[ -x $file ]] && printf "%s %s " -object $file; \
      done \
    ) \
    --instr-profile=target/debug/superrustendo.profdata \
    --show-instantiations --show-line-counts-or-regions \
    --use-color --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='tests/' --ignore-filename-regex='/usr/local/cargo' \
    --Xdemangler=rustfilt -format=html -o target/cov
