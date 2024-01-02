FROM rust:1.75 AS builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build

FROM debian:bookworm-slim AS final

ARG BIN_NAME

RUN useradd --no-create-home --shell /bin/bash app-user
RUN groupadd --users app-user app-group

COPY --from=builder /usr/src/app/target/debug/${BIN_NAME} /srv/app/bin/
COPY --from=builder /usr/src/app/log_config.dev.yml /srv/app/log_config.yml
RUN chown -R app-user:app-group /srv/app

USER app-user
WORKDIR /srv/app

ENV BIN_NAME=${BIN_NAME}
CMD ./bin/${BIN_NAME}