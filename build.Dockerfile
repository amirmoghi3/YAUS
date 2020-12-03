FROM registry.vsf-co.ir/library/rust:1.48.0-alpine3.11 
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/URLS
COPY . .
RUN cargo fetch
RUN cargo install --path .





