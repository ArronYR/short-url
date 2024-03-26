# 使用带有 Node.js 的官方基础镜像
FROM node:18.18.2-buster-slim AS web-builder

# 设置工作目录
WORKDIR /usr/src/app

# 复制项目文件
COPY web/tsconfig*.json ./
COPY web/package.json ./
COPY web/.env* ./
COPY web/index.html ./
COPY web/public ./public
COPY web/src ./src

# 安装依赖
RUN npm config set strict-ssl false
RUN npm install

# 构建应用
RUN npm run build

# 使用 Rust 官方镜像作为构建环境
FROM rust:slim-buster as rs-builder

# 创建一个新的空工作目录
WORKDIR /usr/src/app

# 使用国内的源
COPY config /.cargo/config

# 复制 Cargo.toml 和 Cargo.lock 到工作目录
COPY Cargo.toml Cargo.lock ./

# 复制源代码到工作目录
COPY ./src ./src
COPY ./static ./static
COPY ./templates ./templates

#RUN cargo vendor
RUN cargo clean
RUN cargo build --release

# 使用基础镜像作为运行环境
FROM debian:buster-slim

RUN mkdir /app
WORKDIR /app

# 复制从 builder 阶段构建的可执行文件、静态文件
COPY --from=rs-builder /usr/src/app/target/release/short_url /app/
COPY --from=rs-builder /usr/src/app/static/ /app/static/
COPY --from=rs-builder /usr/src/app/templates/ /app/templates/
COPY --from=web-builder /usr/src/app/dist/ /app/web/

ENV TZ=Asia/Shanghai
ENV RUST_BACKTRACE=full
EXPOSE 80

# 设置容器启动时执行的命令
CMD ["/app/short_url"]
