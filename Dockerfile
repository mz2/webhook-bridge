FROM rustlang/rust:nightly
COPY . /usr/src/app
WORKDIR /usr/src/app
RUN cargo install
ENV ROCKET_ENV=production
EXPOSE $PORT
CMD ROCKET_PORT=$PORT ./target/release/webhook-bridge