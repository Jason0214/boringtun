FROM rust:1.66-alpine AS builder

# Setup apk mirror
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories

# Setup cargo mirror
RUN echo -e "\
[source.crates-io]      \n\
replace-with = 'tuna'   \n\
                        \n\
[source.tuna]           \n\
registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\" \n\
" > /usr/local/cargo/config
RUN apk update

RUN apk add musl-dev

WORKDIR /usr/src/boringtun
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=/usr/src/boringtun/target \
    cargo install boringtun-cli

From nginx:1.23.3-alpine-slim
COPY --from=builder /usr/local/cargo/bin/boringtun-cli /usr/local/bin/boringtun-cli

# Setup apk mirror
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories
RUN apk update

RUN apk add wireguard-tools sudo

RUN echo \#\!/bin/bash > ./start.sh && echo WG_QUICK_USERSPACE_IMPLEMENTATION=boringtun-cli WG_SUDO=1 wg-quick up /wireguard/wg.conf >> ./start.sh && echo nginx -g "'"daemon off";""'" >> ./start.sh
RUN chmod +x ./start.sh
STOPSIGNAL SIGTERM
ENV LOG_LEVEL=debug
CMD ["./start.sh"]
