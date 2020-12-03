FROM alpine:latest 
COPY  target/release/url-shortener /usr/local/bin/url-shortener
CMD ["url-shortener"]