FROM alpine:latest 
COPY  target/release/url-shortener /usr/local/bin/url-shortener
COPY .env.docker .env
CMD ["/usr/local/bin/url-shortener"]