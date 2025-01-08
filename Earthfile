VERSION 0.8

IMPORT github.com/earthly/lib/rust AS rust

FROM rust:1.83.0-slim
WORKDIR /otterbuild

all:
    BUILD +json-format
    BUILD +markdown-format
    BUILD +markdown-lint
    BUILD +yaml-format
    BUILD +yaml-lint

COPY_SOURCES:
    FUNCTION

    # Copy the source code into the container
    COPY . .

node-container:
    FROM node:alpine
    WORKDIR /otterbuild

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

yaml-format:
    FROM +node-container

    # Check the YAML formatting
    RUN prettier --check **/*.{yml,yaml}

yaml-lint:
    FROM pipelinecomponents/yamllint:latest
    WORKDIR /otterbuild

    # Copy the source code into the container
    DO +COPY_SOURCES

    # Check the YAML files for linting errors
    RUN yamllint .
