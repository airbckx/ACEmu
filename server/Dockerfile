# ACEmu servers: login (55000 + HTTP 55001), channel (55999, UDP 55997/55998)
# and admin (55990, serves the web app). Build context is this `server/` folder:
#
#   docker build -t acemu .
#   docker run --network host -e ACEMU_PUBLIC_IP=<this machine's LAN IP> \
#              -v acemu-save:/app/save acemu
#
# See docker-compose.yml for the same thing as a compose file.

# --- admin web app (static export) -----------------------------------------
FROM node:24-alpine AS webapp
WORKDIR /w
COPY webapp/package.json webapp/package-lock.json ./
RUN npm ci
COPY webapp/ ./
RUN npm run build

# --- Rust servers -----------------------------------------------------------
FROM rust:1-slim-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
RUN cargo build --release -p acemu-login -p acemu-channel -p acemu-admin \
    && strip target/release/acemu-login target/release/acemu-channel target/release/acemu-admin

# --- runtime ----------------------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /src/target/release/acemu-login /src/target/release/acemu-channel /src/target/release/acemu-admin ./
COPY --from=webapp /w/out ./webapp/out
COPY data/ ./data/
COPY docker-entrypoint.sh ./
RUN sed -i 's/\r$//' docker-entrypoint.sh && chmod +x docker-entrypoint.sh && mkdir -p save

# save/ holds acemu.db (accounts, players, inventories); keep it on a volume.
VOLUME ["/app/save"]

ENV ACEMU_HOME=/app \
    ACEMU_ADMIN_BIND=0.0.0.0 \
    ACEMU_PUBLIC_IP=127.0.0.1 \
    ACEMU_GUEST_NAT=0 \
    RUST_LOG=info

EXPOSE 55000/tcp 55001/tcp 55999/tcp 55997/udp 55998/udp 55990/tcp

ENTRYPOINT ["./docker-entrypoint.sh"]
