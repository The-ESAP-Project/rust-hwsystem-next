# 作业管理系统后端 - 下一代 (Homework Management System Backend Next)

基于 Rust、Actix Web 构建的高性能作业管理系统后端。该系统提供了完整的作业管理 API，支持学生作业上传、教师批改流程和用户权限管理功能。

## 功能特点

- 💼 **多角色支持**：学生、课代表、教师不同权限管理
- 📝 **作业管理**：处理学生作业上传、批注和评分
- ✅ **教师评阅**：支持老师查看、评价学生作业的 API
- 📊 **数据统计**：提供作业提交情况统计接口
- 🔐 **JWT 授权**：安全的用户认证机制
- 🚀 **高性能**：基于 Rust 的高并发处理能力

## 技术栈

- 🦀 开发语言：[Rust](https://www.rust-lang.org/)
- 🌐 Web 框架：[Actix Web](https://actix.rs/)
- 🗄️ 数据库：[PostgreSQL](https://www.postgresql.org/)
- 🔑 认证：[JWT Token](https://jwt.io/)
- 📝 日志管理：[log](https://crates.io/crates/log)
- 📦 序列化：[Serde](https://serde.rs/)

## 快速开始

### 前置要求

- Rust 1.75.0 或更高版本
- PostgreSQL 14.x 或更高版本

### 环境变量配置

复制示例环境变量文件并根据需要修改：

```bash
cp .env.example .env
```

### 安装依赖

```bash
cargo build
```

### 开发模式运行

```bash
cargo run
```

### 构建生产版本

```bash
cargo build --release
```

### 运行测试

```bash
cargo test
```

## API 接口

### 认证相关

- `POST /api/auth/login` - 用户登录
- `POST /api/auth/refresh` - 刷新令牌

### 用户管理

- `GET /api/users` - 获取用户列表
- `GET /api/users/{id}` - 获取用户详情
- `POST /api/users` - 创建用户
- `PUT /api/users/{id}` - 更新用户信息

### 作业管理

- `GET /api/homework` - 获取作业列表
- `GET /api/homework/{id}` - 获取作业详情
- `POST /api/homework` - 创建新作业
- `PUT /api/homework/{id}` - 更新作业信息
- `DELETE /api/homework/{id}` - 删除作业

### 提交管理

- `GET /api/submissions` - 获取提交列表
- `GET /api/submissions/{id}` - 获取提交详情
- `POST /api/submissions` - 提交作业
- `PUT /api/submissions/{id}` - 更新提交内容
- `PUT /api/submissions/{id}/grade` - 评分提交

## 状态码说明

- `200`: 操作成功
- `400`: 请求参数错误
- `401`: 未认证/认证失败
- `403`: 权限不足
- `404`: 资源不存在
- `500`: 服务器内部错误

## 用户角色权限

- **学生**: 查看个人作业、上传作业
- **课代表**: 学生权限 + 查看全班作业提交情况
- **教师**: 查看并评阅全部学生作业、管理作业

## 部署指南

### Docker 部署

```bash
docker build -t hwsystem-backend .
docker run -p 8080:8080 -d hwsystem-backend
```

### 手动部署

1. 构建发布版本: `cargo build --release`
2. 配置环境变量
3. 运行可执行文件: `./target/release/rust-hwsystem-next`

## 贡献指南

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

## 许可证

本项目基于 GPL-3.0 协议开源。

## 作者

The ESAP Project - [@AptS-1547](https://github.com/AptS-1547)

项目链接: [https://github.com/AptS-1547/rust-hwsystem-next](https://github.com/AptS-1547/rust-hwsystem-next)
