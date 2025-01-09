VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

FROM rust:1.83.0-slim
WORKDIR /blog

all:
    BUILD +json-format
    BUILD +markdown-format
    BUILD +markdown-lint
    BUILD +rust-format
    BUILD +rust-lint
    BUILD +rust-test
    BUILD +yaml-format
    BUILD +yaml-lint

COPY_SOURCES:
    FUNCTION

    # Copy the source code into the container
    COPY . .

COPY_RUST_SOURCES:
    FUNCTION

    # Copy the source code in a cache-friendly way
    COPY --keep-ts Cargo.toml Cargo.lock README.md ./
    COPY --keep-ts --dir crates ./

node-container:
    FROM node:alpine
    WORKDIR /blog

    # Install prettier
    RUN npm install -g prettier markdownlint-cli

    # Copy the source code into the container
    DO +COPY_SOURCES

json-format:
    FROM +node-container

    # Check the JSON formatting
    RUN prettier --check **/*.{json,json5}

markdown-format:
    FROM +node-container

    # Check the formatting of Markdown files
    RUN prettier --check **/*.md

markdown-lint:
    FROM +node-container

    # Check the Markdown files for linting errors
    RUN markdownlint **/*.md

rust-container:
    # Install system-level dependencies
    RUN apt update && apt upgrade -y && apt install -y libssl-dev pkg-config

    # Install clippy and rustfmt
    RUN rustup component add clippy rustfmt

    # Initialize Rust
    DO rust+INIT --keep_fingerprints=true

    # Install cargo-leptosfmt
    DO rust+CARGO --args="install leptosfmt"

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

rust-tarpaulin-container:
    FROM +rust-container

    # Install cargo-tarpaulin
    RUN cargo install cargo-tarpaulin

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

rust-sources:
    FROM +rust-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

rust-build:
    FROM +rust-sources

    # Build the project
    DO rust+CARGO --args="build --all-features --all-targets --locked"

    # Explicitly cache the container at this point
    SAVE IMAGE --cache-hint

rust-format:
    FROM +rust-sources

    # Check the code formatting
    DO rust+CARGO --args="fmt --all --check"

    # Check the HTML formatting
    RUN leptosfmt --check .

rust-lint:
    FROM +rust-build

    # Check the code for linting errors
    DO rust+CARGO --args="clippy --all-targets --all-features --locked -- -D warnings"

rust-publish:
    ARG EXPORT_SITE=""

    FROM +rust-container

    # Install the WASM target
    RUN rustup target add wasm32-unknown-unknown

    # Install trunk to compile the Leptos app
    DO rust+CARGO --args="install trunk"

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Compile the Leptos app
    RUN cd crates/web && trunk build --release

    # Optionally export the compiled app
    IF [ "$EXPORT_SITE" != "" ]
        SAVE ARTIFACT crates/web/dist AS LOCAL crates/web/dist
    END

rust-test:
    # Optionally save the report to the local filesystem
    ARG SAVE_REPORT=""

    FROM +rust-tarpaulin-container

    # Copy the source code in a cache-friendly way
    DO +COPY_RUST_SOURCES

    # Run the tests and measure the code coverage
    DO rust+CARGO --args="tarpaulin \
            --all-features \
            --engine llvm \
            --locked \
            --out Xml \
            --skip-clean \
            --timeout 120 \
            --verbose \
            --workspace"

    # Save the coverage report
    IF [ "$SAVE_REPORT" != "" ]
        SAVE ARTIFACT cobertura.xml AS LOCAL cobertura.xml
    END

yaml-format:
    FROM +node-container

    # Check the YAML formatting
    RUN prettier --check **/*.{yml,yaml}

yaml-lint:
    FROM pipelinecomponents/yamllint:latest
    WORKDIR /blog

    # Copy the source code into the container
    DO +COPY_SOURCES

    # Check the YAML files for linting errors
    RUN yamllint .
