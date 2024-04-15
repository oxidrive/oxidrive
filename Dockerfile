FROM public.ecr.aws/docker/library/golang:1.22-alpine as server-build

WORKDIR /app

COPY go.* .

RUN go mod download

COPY server ./server

RUN go build -o ./bin/oxidrive ./server/cmd/oxidrive

# ========================================================================= #

FROM public.ecr.aws/docker/library/node:20-alpine as css-build

WORKDIR /app

COPY web/input.css .

RUN npx tailwindcss -i ./input.css -o ./output.css

# ========================================================================= #

FROM public.ecr.aws/docker/library/rust:1-slim as web-build

RUN apt-get update && apt-get install wget -y

RUN wget -O /tmp/cargo-binstall.tgz https://github.com/cargo-bins/cargo-binstall/releases/download/v1.6.4/cargo-binstall-x86_64-unknown-linux-musl.tgz && \
    tar -xzf /tmp/cargo-binstall.tgz -C /tmp && \
    mv /tmp/cargo-binstall /usr/bin/cargo-binstall

RUN cargo binstall -y --no-discover-github-token --disable-strategies=compile dioxus-cli@0.5.4

WORKDIR /app

COPY Cargo.* .
COPY web ./web
COPY --from=css-build /app/output.css ./web/assets/styles.css

RUN dx build --release --bin oxidrive

# ========================================================================= #

FROM gcr.io/distroless/static-debian11

ENV OXIDRIVE_ASSETS_FOLDER=/assets

COPY --from=server-build /app/bin/oxidrive /oxidrive
COPY --from=web-build /app/web/dist $OXIDRIVE_ASSETS_FOLDER

CMD ["/oxidrive"]