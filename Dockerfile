FROM rust:latest

WORKDIR /usr/src/camp_review_api
ARG DATABASE_URL=postgresql://postgres:5Cr38sZkDUeDZVYo60zG@containers-us-west-196.railway.app:7984/railway
ENV DATABASE_URL=postgresql://postgres:5Cr38sZkDUeDZVYo60zG@containers-us-west-196.railway.app:7984/railway

COPY . .

RUN cargo install --path .

EXPOSE 8080



CMD ["camp_review_api"]