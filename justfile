set shell := ["bash", "-c"]

alias t := test
alias b := build
alias c := clean
alias l := lint

clean:
    cargo clean

build:
    cargo build

lint: add-clippy
	cargo clippy

fmt: add-fmt
	cargo fmt

run:
    cargo run

test:
	cargo test

add-clippy:
    rustup component add clippy 2> /dev/null

add-fmt:
    rustup component add rustfmt 2> /dev/null

default:
    just --list