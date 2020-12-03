FROM registry.vsf-co.ir/library/debian:buster-slim
COPY  url-shortener .
COPY .env.docker .env
RUN apt-get update && apt-get install -y  pkg-config libpq-dev brotli
CMD ["url-shortener"]