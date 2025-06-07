# 🎓 作业管理系统后端 - 下一代

> 基于 Rust + Actix Web 构建的高性能教育管理平台后端服务

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/actix--web-4.0-blue.svg)](https://actix.rs/)
[![PostgreSQL](https://img.shields.io/badge/postgresql-14%2B-blue.svg)](https://www.postgresql.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## ✨ 产品特色

- 🏫 **教育场景优化** - 专为中小学作业管理设计，支持完整的教学流程
- 👥 **三级权限体系** - 学生、课代表、教师角色精确权限控制
- 📚 **完整作业流程** - 从发布、提交到批改的全链路管理
- 📊 **数据可视化** - 提交统计、成绩分析等多维度数据展示
- 🔐 **企业级安全** - JWT + RBAC 双重认证授权机制
- ⚡ **极致性能** - Rust 原生性能，轻松支持千级并发

## 🛠 技术架构

| 技术栈 | 版本 | 说明 |
|--------|------|------|
| 🦀 **Rust** | 1.75+ | 系统编程语言，内存安全 + 零成本抽象 |
| 🌐 **Actix Web** | 4.x | 高性能异步 Web 框架 |
| 🗄️ **PostgreSQL** | 14+ | 企业级关系型数据库 |
| 🔑 **JWT** | - | 无状态身份认证 |
| 📦 **Serde** | - | 高效序列化/反序列化 |
| 📝 **Tracing** | - | 结构化日志追踪 |

## 🚀 快速开始

### 📋 环境要求

确保你的开发环境满足以下要求：

```bash
# Rust 工具链
rustc --version  # >= 1.75.0
cargo --version

# 数据库
psql --version   # >= 14.0

# 系统依赖 (Ubuntu/Debian)
sudo apt install build-essential libpq-dev
```

### ⚙️ 配置环境

1. **克隆项目**
```bash
git clone https://github.com/The-ESAP-Project/rust-hwsystem-next.git
cd rust-hwsystem-next
```

2. **配置环境变量**
```bash
cp .env.example .env
# 编辑 .env 文件，配置数据库连接等信息
```

3. **数据库初始化**
```bash
# 创建数据库
createdb hwsystem

# 运行迁移脚本
cargo run --bin migrate
```

### 🏃‍♂️ 运行项目

```bash
# 安装依赖
cargo build

# 开发模式
cargo run

# 生产构建
cargo build --release

# 运行测试
cargo test

# 代码检查
cargo clippy
cargo fmt
```

服务启动后访问：`http://localhost:8080`

## 📡 API 文档

### 🔐 认证模块

| 接口 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/api/auth/login` | POST | 用户登录 | 公开 |
| `/api/auth/refresh` | POST | 刷新令牌 | 需认证 |
| `/api/auth/logout` | POST | 用户登出 | 需认证 |

### 👤 用户管理

| 接口 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/api/users` | GET | 获取用户列表 | 教师 |
| `/api/users/{id}` | GET | 获取用户详情 | 自己/教师 |
| `/api/users` | POST | 创建用户 | 教师 |
| `/api/users/{id}` | PUT | 更新用户信息 | 自己/教师 |
| `/api/users/{id}` | DELETE | 删除用户 | 教师 |

### 📝 作业管理

| 接口 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/api/homework` | GET | 获取作业列表 | 所有用户 |
| `/api/homework/{id}` | GET | 获取作业详情 | 所有用户 |
| `/api/homework` | POST | 创建新作业 | 教师 |
| `/api/homework/{id}` | PUT | 更新作业信息 | 教师 |
| `/api/homework/{id}` | DELETE | 删除作业 | 教师 |

### 📋 提交管理

| 接口 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/api/submissions` | GET | 获取提交列表 | 课代表+ |
| `/api/submissions/{id}` | GET | 获取提交详情 | 本人/课代表+ |
| `/api/submissions` | POST | 提交作业 | 学生+ |
| `/api/submissions/{id}` | PUT | 更新提交内容 | 本人 |
| `/api/submissions/{id}/grade` | PUT | 评分提交 | 教师 |

### 📊 统计分析

| 接口 | 方法 | 描述 | 权限 |
|------|------|------|------|
| `/api/stats/overview` | GET | 系统概览统计 | 课代表+ |
| `/api/stats/homework/{id}` | GET | 单个作业统计 | 课代表+ |
| `/api/stats/student/{id}` | GET | 学生作业统计 | 本人/教师 |

## 🔑 权限体系

### 角色定义

| 角色 | 权限说明 |
|------|----------|
| 👨‍🎓 **学生** | 查看作业、提交作业、查看个人成绩 |
| 👑 **课代表** | 学生权限 + 查看班级提交统计、催交提醒 |
| 👨‍🏫 **教师** | 全部权限：用户管理、作业管理、批改评分 |

### 数据访问控制

- **水平权限**：用户只能访问自己的数据
- **垂直权限**：高级角色可访问低级角色功能
- **资源隔离**：按班级/课程进行数据隔离

## 🏗 部署指南

### 🐳 Docker 部署（推荐）

```bash
# 使用 Docker Compose
docker-compose up -d

# 或单独构建
docker build -t hwsystem-backend .
docker run -p 8080:8080 -d hwsystem-backend
```

### ☁️ 云原生部署

```yaml
# kubernetes deployment example
apiVersion: apps/v1
kind: Deployment
metadata:
  name: hwsystem-backend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: hwsystem-backend
  template:
    spec:
      containers:
      - name: backend
        image: hwsystem-backend:latest
        ports:
        - containerPort: 8080
```

### 🔧 生产环境配置

```bash
# 构建优化版本
RUSTFLAGS="-C target-cpu=native" cargo build --release

# 使用系统服务管理
sudo systemctl enable hwsystem-backend
sudo systemctl start hwsystem-backend
```

## 🧪 测试指南

```bash
# 单元测试
cargo test --lib

# 集成测试
cargo test --test integration

# 基准测试
cargo bench

# 测试覆盖率
cargo tarpaulin --out Html
```

## 📊 性能指标

| 指标 | 数值 | 说明 |
|------|------|------|
| 🚀 **响应时间** | < 50ms | P95 API 响应时间 |
| 🔄 **并发处理** | 1000+ | 单机并发连接数 |
| 💾 **内存占用** | < 100MB | 空载内存使用 |
| ⚡ **启动时间** | < 2s | 冷启动到服务就绪 |

### 提交规范

```bash
# 功能开发
git commit -m "feat: 添加作业批量导入功能"

# Bug 修复
git commit -m "fix: 修复用户权限检查逻辑"

# 文档更新
git commit -m "docs: 更新 API 接口文档"
```

### 开发工作流

1. 🍴 Fork 本仓库
2. 🌿 创建特性分支 `git checkout -b feature/amazing-feature`
3. 💾 提交更改 `git commit -m 'feat: add amazing feature'`
4. 📤 推送分支 `git push origin feature/amazing-feature`
5. 🔄 创建 Pull Request

## 📄 许可证

本项目采用 [GPL-3.0](LICENSE) 许可证开源。

## 👥 团队

**The ESAP Project**
- 🔗 GitHub: [@AptS-1547](https://github.com/AptS-1547)
- 📧 Email: contact@esap-project.org
- 🌐 项目主页: [https://github.com/The-ESAP-Project/rust-hwsystem-next](https://github.com/The-ESAP-Project/rust-hwsystem-next)

## 🙏 致谢

- Rust 社区提供的优秀生态
- Actix Web 框架的稳定支持
- 所有贡献者的辛勤付出

---

<div align="center">

**如果这个项目对你有帮助，请给我们一个 ⭐️ Star！**

[报告 Bug](https://github.com/The-ESAP-Project/rust-hwsystem-next/issues) · 
[功能建议](https://github.com/The-ESAP-Project/rust-hwsystem-next/issues) · 
[加入讨论](https://github.com/The-ESAP-Project/rust-hwsystem-next/discussions)

</div>
