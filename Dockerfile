FROM registry.vsf-co.ir/library/debian:buster-slim
COPY  url-shortener .
COPY .env.docker .env
ENTRYPOINT [ "./url-shortener"]