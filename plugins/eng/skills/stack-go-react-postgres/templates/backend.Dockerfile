# <project>/backend/Dockerfile
#
# `build: ./backend` in the compose file needs this, and scripts/version-check.sh reads the
# Go version out of the first line. ⚠ Keep the tag exact — a floating `golang:1-alpine`
# cannot be checked against go.mod.

FROM golang:1.27.0-alpine AS build
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download
COPY . .
# ⚠ CGO_ENABLED=0 is what makes distroless/static possible — but `go test -race` REQUIRES
#   cgo and a C toolchain. A CI image built only for CGO_ENABLED=0 cannot run the test gate.
RUN CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o /api ./cmd/api

FROM gcr.io/distroless/static-debian12:nonroot
COPY --from=build /api /api
USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/api"]
