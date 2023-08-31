FROM rust:latest

WORKDIR /usr/src/camp_review_api

COPY . .

RUN cargo install --path .


EXPOSE 8080

CMD ["camp_review_api"]