FROM public.ecr.aws/docker/library/golang:1.22-alpine as server-build

WORKDIR /app

COPY server/go.* .

RUN go mod download

COPY server .

RUN go build -o ./bin/oxidrive ./cmd/oxidrive

# ========================================================================= #

FROM public.ecr.aws/docker/library/node:20-alpine as css-build

WORKDIR /app

COPY web/input.css .

RUN npx tailwindcss -i ./input.css -o ./output.css

FROM public.ecr.aws/docker/library/rust:1-slim as web-build

RUN apt-get update && apt-get install -y pkg-config libssl-dev perl make \
  && cargo install dioxus-cli --locked && rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY web .
COPY --from=css-build /app/output.css ./assets/styles.css

RUN dx build --release

# ========================================================================= #

FROM gcr.io/distroless/static-debian11

ENV OXIDRIVE_ASSETS_FOLDER=/assets

COPY --from=server-build /app/bin/oxidrive /oxidrive
COPY --from=web-build /app/dist $OXIDRIVE_ASSETS_FOLDER

CMD ["/oxidrive"]