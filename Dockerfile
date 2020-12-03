FROM alpine:latest 
COPY  url-shortener /usr/local/bin/url-shortener
WORKDIR /usr/local/bin
CMD ["url-shortener"]