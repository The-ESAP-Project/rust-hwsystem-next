# 使用官方 Rust 镜像作为构建环境
FROM rust:1.86-slim AS builder

# 创建新的空项目
WORKDIR /usr/src/app
RUN cargo init

# 首先复制 Cargo.toml 和 Cargo.lock (如果存在)
COPY Cargo.toml ./
# 尝试复制 Cargo.lock，如果不存在则忽略
COPY Cargo.lock* ./

# 构建一个空项目以缓存依赖
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release && \
    rm src/*.rs

# 复制源代码
COPY src/ ./src/

# 构建实际应用
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    touch src/main.rs && \
    cargo build --release

# 第二阶段：创建运行环境
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 创建非特权用户
RUN groupadd -r appuser && useradd -r -g appuser appuser

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /usr/src/app/target/release/rust-hwsystem-next /app/rust-hwsystem-next

# 切换到非特权用户
USER appuser

# 设置环境变量
ENV RUST_LOG=info

# 暴露应用端口
EXPOSE 8080

# 运行应用
CMD ["./rust-hwsystem-next"]
