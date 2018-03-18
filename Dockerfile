FROM rustlang/rust:nightly

COPY . /usr/src/app
WORKDIR /usr/src/app

RUN cargo install

ENV PORT 3000
ENV ROCKET_HANGOUT_SPACE 'replace-hangout-space'
ENV ROCKET_HANGOUT_TOKEN 'replace-hangout-token'
ENV ROCKET_HANGOUT_KEY 'replace-hangout-key'

EXPOSE $PORT

CMD ROCKET_PORT=$PORT ./target/release/webhook-bridge