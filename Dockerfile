FROM ghcr.io/redocly/cli as openapi

WORKDIR /app

COPY server/openapi .

RUN redocly join -o out.yml openapi.yaml ./*.yaml

# ========================================================================= #

FROM golang:1.22-alpine as server-build

WORKDIR /app

COPY go.* .

RUN go mod download

COPY --from=openapi /app/out.yml ./server/openapi/out.yml
COPY server ./server

RUN go generate ./server/...
RUN go build -o ./bin/oxidrive ./server/cmd/oxidrive

# ========================================================================= #

FROM node:20-alpine as css-build

WORKDIR /app

COPY web/app/package*.json .

RUN npm ci

COPY web/app .

RUN npx tailwindcss -i ./input.css -o ./output.css

# ========================================================================= #

FROM rust:1-slim as web-build

RUN apt-get update && apt-get install wget -y

RUN wget -O /tmp/cargo-binstall.tgz https://github.com/cargo-bins/cargo-binstall/releases/download/v1.6.4/cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    tar -xzf /tmp/cargo-binstall.tgz -C /tmp && \
    mv /tmp/cargo-binstall /usr/bin/cargo-binstall

RUN cargo binstall -y --no-discover-github-token --disable-strategies=compile dioxus-cli@0.5.4

WORKDIR /app

COPY Cargo.* .
COPY web ./web
COPY --from=css-build /app/output.css ./web/app/assets/styles.css

RUN cd web/app && dx build --release

# ========================================================================= #

FROM gcr.io/distroless/static-debian11

ENV OXIDRIVE_ASSETS_FOLDER=/assets

COPY --from=server-build /app/bin/oxidrive /oxidrive
COPY --from=web-build /app/web/app/dist $OXIDRIVE_ASSETS_FOLDER

CMD ["/oxidrive"]
