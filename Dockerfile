# 使用 Rust 官方镜像作为构建环境
FROM rust:slim-buster as builder

# 创建一个新的空工作目录
WORKDIR /usr/src/app

# 复制 Cargo.toml 和 Cargo.lock 到工作目录
COPY Cargo.toml Cargo.lock ./

# 使用国内的源
COPY config /.cargo/config

# 构建依赖项，这样如果依赖没变，就可以利用 Docker 缓存
RUN mkdir src/
# 复制源代码到工作目录
COPY src/ ./src/
COPY static/ ./static/
COPY templates/ ./templates/

RUN cargo build --release

# 使用基础镜像作为运行环境
FROM debian:buster-slim

RUN mkdir /app
WORKDIR /app

# 复制从 builder 阶段构建的可执行文件、静态文件
COPY --from=builder /usr/src/app/target/release/short_url /app/
COPY --from=builder /usr/src/app/static/ /app/static/
COPY --from=builder /usr/src/app/templates/ /app/templates/

ENV TZ=Asia/Shanghai
ENV RUST_BACKTRACE=full

# 设置容器启动时执行的命令
CMD ["/app/short_url"]
