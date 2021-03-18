FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /app
ADD --chown=rust:rust . /app/xunit-repo-client
WORKDIR /app/xunit-repo-client
RUN rm -rf target
RUN cargo build --release --features=libsqlite3-sys

FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /app/xunit-repo-client/target/x86_64-unknown-linux-musl/release/xunit-repo-client /usr/bin/xunit-repo-client

#RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
