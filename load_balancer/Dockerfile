FROM golang:1.23.5-alpine

WORKDIR /app

COPY go.mod go.sum ./

RUN cat go.mod && cat go.sum && go mod download

COPY . .

RUN CGO_ENABLED=0 GOOS=linux go build -o main .

FROM alpine:latest

WORKDIR /root/

COPY --from=0 /app/main .

EXPOSE 3000
EXPOSE 8080

CMD ["./main"]
