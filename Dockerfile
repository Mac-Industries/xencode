FROM alpine:latest

RUN apk add --no-cache sqlite

VOLUME /data

WORKDIR /data

CMD ["sqlite3", "ideas.db"]