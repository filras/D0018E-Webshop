FROM rust

WORKDIR /usr/src/app


COPY . .

RUN apt-get update && apt-get install -y npm

RUN cargo build
RUN cd frontend &&  npm i

ENV PORT=8000

EXPOSE 8000

CMD cargo run && cd frontend && npm run dev
