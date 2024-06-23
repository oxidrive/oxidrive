FROM node:20-alpine as node-base

WORKDIR /app

COPY package*.json .

RUN npm ci

# ========================================================================= #

FROM node-base as openapi

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

FROM node-base as web-build

WORKDIR /app

COPY web ./web

RUN cd web && npx svelte-kit sync && npx vite build

# ========================================================================= #

FROM gcr.io/distroless/static-debian11

ENV OXIDRIVE_ASSETS_FOLDER=/assets
ENV HOST=0.0.0.0

COPY --from=server-build /app/bin/oxidrive /oxidrive
COPY --from=web-build /app/web/build $OXIDRIVE_ASSETS_FOLDER

ENTRYPOINT ["/oxidrive"]
