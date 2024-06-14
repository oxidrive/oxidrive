FROM node:20-alpine as npm-tools

WORKDIR /app

COPY package*.json .

RUN npm ci

# ========================================================================= #

FROM npm-tools as openapi

WORKDIR /app

COPY server/openapi .

RUN npx redocly join -o out.yml openapi.yaml ./*.yaml

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

FROM npm-tools as css-build

WORKDIR /app

COPY web/app .

RUN npx tailwindcss -i ./input.css -o ./output.css

# ========================================================================= #

FROM ghcr.io/oxidrive/ci/dioxus-cli:0.5.4 as web-build

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
