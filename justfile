default:
    just --list

lint:
    just fmt
    just clippy
    just machete

fmt:
    cargo +nightly fmt

clippy:
    cargo clippy

machete:
    cargo machete --with-metadata
