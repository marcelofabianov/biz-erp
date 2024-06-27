FROM golang:1.23rc-bookworm

WORKDIR /usr/src/app

RUN apt-get update -yqq && \
  apt-get upgrade -y && \
  apt-get install -y --no-install-recommends \
  git \
  wget \
  curl \
  unzip \
  nano \
  tree \
  build-essential \
  librdkafka-dev \
  libprotobuf-dev \
  protobuf-compiler && \
  rm -rf /var/lib/apt/lists/*

COPY . .

RUN go mod download && go mod verify
RUN go build -v -o /usr/src/app ./...

EXPOSE 8080

CMD ["./app"]
