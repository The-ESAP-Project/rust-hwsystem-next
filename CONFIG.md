# 配置系统使用说明

本项目现在使用 TOML 格式的配置文件来管理所有配置项。

## 配置文件

### 主要配置文件
- `config.toml` - 默认配置文件
- `config.development.toml` - 开发环境配置
- `config.production.toml` - 生产环境配置
- `config.example.toml` - 配置文件示例和说明

### 配置加载顺序
1. 首先加载 `config.toml`（如果存在）
2. 然后根据 `APP_ENV` 环境变量加载对应的环境配置文件
3. 最后使用环境变量覆盖配置项

## 环境变量支持

### 新格式（推荐）
使用 `HWSYSTEM_` 前缀的环境变量：
```bash
HWSYSTEM_SERVER_PORT=9000
HWSYSTEM_JWT_SECRET=my_secret_key
HWSYSTEM_DATABASE_URL=postgresql://user:pass@localhost/db
```

### 传统格式（向后兼容）
继续支持原有的环境变量名：
```bash
APP_ENV=production
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
JWT_SECRET=my_secret_key
DATABASE_URL=hwsystem.db
REDIS_URL=redis://localhost:6379/
```

## 快速开始

1. 复制示例配置文件：
```bash
cp config.example.toml config.toml
```

2. 根据需要修改配置

3. 或者使用环境变量：
```bash
export APP_ENV=development
export JWT_SECRET=your_secret_key
```

## 主要配置项

### 应用设置
- `app.environment`: 运行环境 (development/production)
- `app.log_level`: 日志级别 (trace/debug/info/warn/error)

### 服务器设置
- `server.host`: 服务器主机
- `server.port`: 服务器端口
- `server.workers`: 工作线程数 (0=自动)

### JWT 设置
- `jwt.secret`: JWT 密钥
- `jwt.access_token_expiry`: Access Token 过期时间(分钟)
- `jwt.refresh_token_expiry`: Refresh Token 过期时间(天)
- `jwt.refresh_token_remember_me_expiry`: Refresh Token 记住我选项有效期(天)

### 数据库设置
- `database.backend`: 数据库类型 (sqlite/postgres/mysql)
- `database.url`: 数据库连接字符串

### 缓存设置
- `cache.type`: 缓存类型 (memory/redis)
- `cache.redis.url`: Redis 连接字符串
