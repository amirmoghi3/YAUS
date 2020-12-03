FROM registry.vsf-co.ir/library/rust:latest
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/URLS
COPY . .
RUN cargo fetch
RUN cargo build --release.





