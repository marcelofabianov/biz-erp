FROM golang:1.23rc1-bookworm

WORKDIR /app

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

EXPOSE 8080

CMD ["tail", "-f", "/dev/null"]
