# 🎓 作业管理系统后端 - 下一代

> 基于 Rust + Actix Web 构建的高性能教育管理平台后端服务

[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs/)
[![PostgreSQL](https://img.shields.io/badge/postgresql-14%2B-blue.svg)](https://www.postgresql.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

---

## ✨ 产品特色

- 🏫 教育场景优化，完整教学流程支持
- 👥 三级权限体系：学生、课代表、教师
- 📚 作业全流程管理：发布、提交、批改
- 📊 数据可视化：统计分析、成绩展示
- 🔐 企业级安全：JWT + RBAC 双重认证
- ⚡ 极致性能：Rust 原生高并发

## 🛠 技术架构

| 技术栈         | 版本   | 说明                   |
| -------------- | ------ | ---------------------- |
| 🦀 Rust        | 1.88+  | 系统编程语言           |
| 🌐 Actix Web   | 4.x    | 高性能异步 Web 框架    |
| 🗄️ PostgreSQL  | 14+    | 企业级关系型数据库     |
| 🔑 JWT         | -      | 无状态身份认证         |
| 📦 Serde       | -      | 高效序列化/反序列化    |
| 📝 Tracing     | -      | 结构化日志追踪         |

## 🚀 快速开始

### 环境要求

- Rust 工具链 >= 1.88.0
- PostgreSQL >= 14.0
- 系统依赖：`build-essential libpq-dev`

### 配置环境

1. 克隆项目  
   `git clone https://github.com/The-ESAP-Project/rust-hwsystem-next.git && cd rust-hwsystem-next`
2. 配置环境变量  
   `cp config.example.toml config.toml`  
   或使用环境变量（详见 [CONFIG.md](CONFIG.md)）
3. 初始化数据库  
   `createdb hwsystem`  
   `cargo run --bin migrate`
4. 运行项目  
   `cargo run`

### 运行与开发

```bash
cargo build         # 构建项目
cargo run           # 开发模式运行
cargo build --release # 生产构建
cargo test          # 运行测试
cargo clippy        # 代码检查
cargo fmt           # 格式化代码
```

服务启动后访问：`http://localhost:8080`

## 📡 API 文档

详见 [API 文档](#api-文档) 或接口自动文档。

### 认证模块

| 接口                | 方法 | 描述       | 权限   |
|---------------------|------|------------|--------|
| `/api/auth/login`   | POST | 用户登录   | 公开   |
| `/api/auth/refresh` | POST | 刷新令牌   | 需认证 |
| `/api/auth/logout`  | POST | 用户登出   | 需认证 |

### 用户管理

| 接口                | 方法 | 描述         | 权限     |
|---------------------|------|--------------|----------|
| `/api/users`        | GET  | 获取用户列表 | 教师     |
| `/api/users/{id}`   | GET  | 获取用户详情 | 自己/教师|
| `/api/users`        | POST | 创建用户     | 教师     |
| `/api/users/{id}`   | PUT  | 更新用户信息 | 自己/教师|
| `/api/users/{id}`   | DELETE | 删除用户   | 教师     |

### 作业管理

| 接口                   | 方法 | 描述         | 权限   |
|------------------------|------|--------------|--------|
| `/api/homework`        | GET  | 获取作业列表 | 所有用户 |
| `/api/homework/{id}`   | GET  | 获取作业详情 | 所有用户 |
| `/api/homework`        | POST | 创建新作业   | 教师   |
| `/api/homework/{id}`   | PUT  | 更新作业信息 | 教师   |
| `/api/homework/{id}`   | DELETE | 删除作业   | 教师   |

### 提交管理

| 接口                          | 方法 | 描述         | 权限         |
|-------------------------------|------|--------------|--------------|
| `/api/submissions`            | GET  | 获取提交列表 | 课代表+      |
| `/api/submissions/{id}`       | GET  | 获取提交详情 | 本人/课代表+ |
| `/api/submissions`            | POST | 提交作业     | 学生+        |
| `/api/submissions/{id}`       | PUT  | 更新提交内容 | 本人         |
| `/api/submissions/{id}/grade` | PUT  | 评分提交     | 教师         |

### 统计分析

| 接口                          | 方法 | 描述         | 权限         |
|-------------------------------|------|--------------|--------------|
| `/api/stats/overview`         | GET  | 系统概览统计 | 课代表+      |
| `/api/stats/homework/{id}`    | GET  | 单个作业统计 | 课代表+      |
| `/api/stats/student/{id}`     | GET  | 学生作业统计 | 本人/教师    |

## 🔑 权限体系

- 学生：查看/提交作业、查看成绩
- 课代表：学生权限 + 统计、提醒
- 教师：全部权限，管理用户和作业

## 🏗 部署指南

- Docker 部署：  
  `docker-compose up -d`  
  或  
  `docker build -t hwsystem-backend . && docker run -p 8080:8080 -d hwsystem-backend`
- 云原生部署：参考 Kubernetes 示例

## 🧪 测试指南

```bash
cargo test --lib           # 单元测试
cargo test --test integration # 集成测试
cargo bench                # 基准测试
cargo tarpaulin --out Html # 测试覆盖率
```

## 📊 性能指标

| 指标         | 数值      | 说明           |
|--------------|-----------|----------------|
| 响应时间     | < 50ms    | P95 API 响应   |
| 并发处理     | 1000+     | 单机连接数     |
| 内存占用     | < 100MB   | 空载内存       |
| 启动时间     | < 2s      | 冷启动到就绪   |

## 🤝 开发贡献

欢迎 PR！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)。

## 📄 许可证

本项目采用 [MIT 许可证](LICENSE)。

---

<div align="center">

如果这个项目对你有帮助，请给我们一个 ⭐️ Star！

[报告 Bug](https://github.com/The-ESAP-Project/rust-hwsystem-next/issues) · 
[功能建议](https://github.com/The-ESAP-Project/rust-hwsystem-next/issues) · 
[加入讨论](https://github.com/The-ESAP-Project/rust-hwsystem-next/discussions)

</div>