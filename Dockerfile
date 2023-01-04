FROM rust:1.66-alpine

WORKDIR /usr/src/boringtun
COPY . .

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apk/repositories
RUN apk update
RUN apk add wireguard-tools musl-dev sudo

RUN echo -e "\
[source.crates-io]      \n\
replace-with = 'tuna'   \n\
                        \n\
[source.tuna]           \n\
registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\" \n\
" > /usr/local/cargo/config

RUN cargo install boringtun-cli
