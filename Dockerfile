FROM alpine:latest 
COPY  target/release/url-shortener /usr/local/bin/url-shortener
WORKDIR /usr/local/bin
CMD ["url-shortener"]