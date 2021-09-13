FROM rust:1.48

WORKDIR /home
COPY . .
EXPOSE 8080

ENV TZ=Europe/Warsaw
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y sqlite3

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN diesel migration run
RUN cargo build

CMD cargo run