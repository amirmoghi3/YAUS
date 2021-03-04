FROM registry.vnfco.ir/library/debian:buster-slim
COPY  url-shortener .
COPY .env.docker .env
EXPOSE 8000
ENTRYPOINT [ "./url-shortener"]