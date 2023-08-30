FROM rust:latest

WORKDIR /usr/src/camp_reviews

COPY . .

RUN cargo install --path .


EXPOSE 8080

CMD ["camp_reviews"]