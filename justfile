#!/usr/bin/env just
set positional-arguments

default:
    @just --list

install-deps:
    npm install

build:
    npx sass -s compressed media/scss/main.scss css-js/main.css
    cargo build --release

run *args:
    target/release/mpr "${@}"