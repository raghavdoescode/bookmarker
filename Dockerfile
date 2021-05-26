FROM rust

WORKDIR /usr/bot

COPY . .

RUN apt install openssl 
RUN cargo build --release

ENV TOKEN=sometoken

CMD ["./target/release/bookmarker"]
