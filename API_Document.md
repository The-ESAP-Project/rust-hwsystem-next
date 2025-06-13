# 作业管理系统 API 文档

## 概述

作业管理系统是一个基于 Rust 构建的 Web 应用程序，提供完整的作业创建、提交、批改和管理功能。

### 基础信息

- **Base URL**: `http://localhost:8080/api/v1`
- **认证方式**: JWT Bearer Token
- **数据格式**: JSON
- **API 版本**: v1.0

### 统一响应格式

```json
{
  "code": 0,
  "message": "string",
  "data": {},
  "timestamp": "2025-06-13T10:30:00Z"
}
```

### 状态码说明

| 状态码 | 说明         |
| ------ | ------------ |
| 0      | 成功         |
| 1001   | 参数错误     |
| 1002   | 认证失败     |
| 1003   | 权限不足     |
| 1004   | 资源不存在   |
| 1005   | 系统内部错误 |

---

## 1. 认证管理

### 1.1 用户登录

**接口地址**: `POST /auth/login`

**请求参数**:

```json
{
  "username": "student001",
  "password": "password123"
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "登录成功",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "id": 1001,
      "username": "student001",
      "role": "student",
      "email": "student001@example.com"
    }
  }
}
```

### 1.2 刷新令牌

**接口地址**: `POST /auth/refresh`

**请求头**: `Authorization: Bearer {refresh_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "令牌刷新成功",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
  },
  "timestamp": "2025-06-13T10:30:00Z"
}
```

### 1.3 获取用户信息

**接口地址**: `GET /auth/me`

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "获取用户信息成功",
  "data": {
    "id": 1001,
    "username": "student001",
    "role": "student",
    "email": "student001@example.com",
    "profile": {
      "name": "张三",
      "student_id": "2025001",
      "class": "计算机科学2025级1班"
    },
    "created_at": "2025-01-01T00:00:00Z",
    "last_login": "2025-06-13T10:30:00Z",
    "status": "active"
  },
  "timestamp": "2025-06-13T10:30:00Z"
}
```

### 1.4 用户退出

在 JWT 模式下，用户退出主要由客户端处理：

1. 删除本地存储的 access_token 和 refresh_token
2. 清除相关的用户状态和缓存
3. 重定向到登录页面

---

## 2. 作业管理

### 2.1 创建作业

**接口地址**: `POST /homeworks`

**权限要求**: 教师或管理员

**请求头**:

- `Authorization: Bearer {access_token}`
- `Content-Type: multipart/form-data`

**请求参数**:

- `title`: 作业标题 (必填)
- `description`: 作业描述 (可选)
- `content`: 作业详细内容 (必填)
- `deadline`: 截止时间 (必填, ISO 8601 格式)
- `max_score`: 最高分数 (可选, 默认 100)
- `allow_late_submission`: 是否允许迟交 (可选, 默认 false)
- `attachments[]`: 附件文件 (可选, 支持多文件)

**响应示例**:

```json
{
  "code": 0,
  "message": "作业创建成功",
  "data": {
    "id": 2001,
    "title": "数据结构作业",
    "deadline": "2025-07-01T23:59:59Z",
    "status": "published"
  }
}
```

### 2.2 获取作业列表

**接口地址**: `GET /homeworks`

**请求头**: `Authorization: Bearer {access_token}`

**查询参数**:

- `page`: 页码 (默认: 1)
- `size`: 每页数量 (默认: 10)
- `status`: 作业状态筛选 (draft, published, closed)
- `search`: 搜索关键词
- `order_by`: 排序字段 (created_at, deadline, title)
- `order`: 排序方向 (asc, desc)

**响应示例**:

```json
{
  "code": 0,
  "message": "获取作业列表成功",
  "data": {
    "items": [
      {
        "id": 2001,
        "title": "数据结构作业 - 第三章",
        "description": "完成二叉树相关习题",
        "deadline": "2025-07-01T23:59:59Z",
        "max_score": 100,
        "submission_count": 25,
        "status": "published",
        "created_at": "2025-06-13T10:30:00Z",
        "created_by": {
          "id": 5001,
          "username": "teacher_zhang",
          "name": "张老师"
        }
      }
    ],
    "pagination": {
      "page": 1,
      "size": 10,
      "total": 15,
      "pages": 2
    }
  },
  "timestamp": "2025-06-13T10:30:00Z"
}
```

### 2.3 获取作业详情

**接口地址**: `GET /homeworks/{homework_id}`

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "获取作业详情成功",
  "data": {
    "id": 2001,
    "title": "数据结构作业 - 第三章",
    "description": "完成二叉树相关习题，包括遍历算法实现",
    "content": "1. 实现二叉树的前序遍历\n2. 实现二叉树的中序遍历\n3. 实现二叉树的后序遍历",
    "deadline": "2025-07-01T23:59:59Z",
    "max_score": 100,
    "allow_late_submission": true,
    "attachments": [
      {
        "id": "file_abc123",
        "filename": "assignment_template.pdf",
        "size": 1024000,
        "download_url": "/api/v1/files/download/abc123"
      }
    ],
    "submission_count": 25,
    "status": "published",
    "created_by": {
      "id": 5001,
      "username": "teacher_zhang",
      "name": "张老师"
    },
    "created_at": "2025-06-13T10:30:00Z",
    "updated_at": "2025-06-13T10:30:00Z"
  },
  "timestamp": "2025-06-13T10:30:00Z"
}
```

### 2.4 更新作业

**接口地址**: `PUT /homeworks/{homework_id}`

**权限要求**: 作业创建者或管理员

**请求头**: `Authorization: Bearer {access_token}`

**请求参数**:

```json
{
  "title": "数据结构作业 - 第三章（更新版）",
  "description": "更新后的作业描述",
  "content": "更新后的作业内容",
  "deadline": "2025-07-05T23:59:59Z",
  "allow_late_submission": false
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "作业更新成功",
  "data": {
    "id": 2001,
    "title": "数据结构作业 - 第三章（更新版）",
    "deadline": "2025-07-05T23:59:59Z",
    "updated_at": "2025-06-13T15:30:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 2.5 删除作业

**接口地址**: `DELETE /homeworks/{homework_id}`

**权限要求**: 作业创建者或管理员

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "作业删除成功",
  "timestamp": "2025-06-13T15:30:00Z"
}
```

---

## 3. 作业提交管理

### 3.1 提交作业

**接口地址**: `POST /homeworks/{homework_id}/submissions`

**权限要求**: 学生

**请求头**:

- `Authorization: Bearer {access_token}`
- `Content-Type: multipart/form-data`

**请求参数**:

- `content`: 作业答案内容 (必填)
- `attachments[]`: 附件文件 (可选, 支持多文件)

**响应示例**:

```json
{
  "code": 0,
  "message": "作业提交成功",
  "data": {
    "id": 3001,
    "homework_id": 2001,
    "student_id": 1001,
    "content": "作业答案内容...",
    "attachments": [
      {
        "id": "file_abc123",
        "filename": "homework_solution.py",
        "size": 2048,
        "download_url": "/api/v1/files/download/abc123"
      }
    ],
    "submitted_at": "2025-06-13T15:30:00Z",
    "status": "submitted",
    "is_late": false
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 3.2 获取提交列表

**接口地址**: `GET /homeworks/{homework_id}/submissions`

**权限要求**: 教师或管理员

**请求头**: `Authorization: Bearer {access_token}`

**查询参数**:

- `page`: 页码 (默认: 1)
- `size`: 每页数量 (默认: 10)
- `status`: 提交状态筛选 (submitted, graded, late)
- `order_by`: 排序字段 (submitted_at, student_name, score)
- `order`: 排序方向 (asc, desc)

**响应示例**:

```json
{
  "code": 0,
  "message": "获取提交列表成功",
  "data": {
    "items": [
      {
        "id": 3001,
        "student": {
          "id": 1001,
          "username": "student001",
          "name": "张三",
          "student_id": "2025001"
        },
        "submitted_at": "2025-06-13T15:30:00Z",
        "status": "graded",
        "score": 95,
        "is_late": false,
        "has_feedback": true
      }
    ],
    "pagination": {
      "page": 1,
      "size": 10,
      "total": 25,
      "pages": 3
    },
    "statistics": {
      "total_submissions": 25,
      "graded_count": 20,
      "average_score": 87.5,
      "late_submissions": 3
    }
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 3.3 获取提交详情

**接口地址**: `GET /submissions/{submission_id}`

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

````json
{
  "code": 0,
  "message": "获取提交详情成功",
  "data": {
    "id": 3001,
    "homework": {
      "id": 2001,
      "title": "数据结构作业 - 第三章",
      "max_score": 100
    },
    "student": {
      "id": 1001,
      "username": "student001",
      "name": "张三",
      "student_id": "2025001"
    },
    "content": "## 作业答案\n\n### 1. 前序遍历实现\n```python\ndef preorder(root):\n    if not root:\n        return []\n    return [root.val] + preorder(root.left) + preorder(root.right)\n```",
    "attachments": [
      {
        "id": "file_abc123",
        "filename": "homework_solution.py",
        "size": 2048,
        "download_url": "/api/v1/files/download/abc123"
      }
    ],
    "submitted_at": "2025-06-13T15:30:00Z",
    "status": "graded",
    "score": 95,
    "feedback": {
      "content": "代码实现正确，逻辑清晰。建议优化递归边界条件的处理。",
      "graded_by": {
        "id": 5001,
        "username": "teacher_zhang",
        "name": "张老师"
      },
      "graded_at": "2025-06-14T10:00:00Z"
    },
    "is_late": false
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
````

### 3.4 批改作业

**接口地址**: `POST /submissions/{submission_id}/feedback`

**权限要求**: 教师或管理员

**请求头**: `Authorization: Bearer {access_token}`

**请求参数**:

```json
{
  "score": 95,
  "feedback": "代码实现正确，逻辑清晰。建议优化递归边界条件的处理。\n\n## 评分详情\n- 算法正确性: 30/30\n- 代码规范性: 25/25\n- 注释完整性: 20/20\n- 创新性: 20/25",
  "send_notification": true
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "批改完成",
  "data": {
    "submission_id": 3001,
    "score": 95,
    "feedback": "代码实现正确，逻辑清晰...",
    "graded_by": {
      "id": 5001,
      "username": "teacher_zhang",
      "name": "张老师"
    },
    "graded_at": "2025-06-14T10:00:00Z"
  },
  "timestamp": "2025-06-14T10:00:00Z"
}
```

### 3.5 导出提交统计

**接口地址**: `POST /homeworks/{homework_id}/submissions/export`

**权限要求**: 教师或管理员

**请求头**: `Authorization: Bearer {access_token}`

**请求参数**:

```json
{
  "format": "xlsx",
  "include_fields": [
    "student_name",
    "student_id",
    "submitted_at",
    "score",
    "is_late"
  ],
  "filter": {
    "status": "graded",
    "min_score": 60
  }
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "导出任务已创建",
  "data": {
    "task_id": "export_task_123",
    "download_url": "/api/v1/files/download/export_task_123",
    "expires_at": "2025-06-14T10:00:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

---

## 4. 用户管理

### 4.1 获取用户列表

**接口地址**: `GET /users`

**权限要求**: 管理员

**请求头**: `Authorization: Bearer {access_token}`

**查询参数**:

- `page`: 页码 (默认: 1)
- `size`: 每页数量 (默认: 10)
- `role`: 用户角色筛选 (student, teacher, admin)
- `status`: 用户状态筛选 (active, inactive, suspended)
- `search`: 搜索关键词 (用户名、邮箱、姓名)

**响应示例**:

```json
{
  "code": 0,
  "message": "获取用户列表成功",
  "data": {
    "items": [
      {
        "id": 1001,
        "username": "student001",
        "email": "student001@example.com",
        "role": "student",
        "status": "active",
        "profile": {
          "name": "张三",
          "student_id": "2025001",
          "class": "计算机科学2025级1班"
        },
        "last_login": "2025-06-13T15:30:00Z",
        "created_at": "2025-01-01T00:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "size": 10,
      "total": 150,
      "pages": 15
    }
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 4.2 创建用户

**接口地址**: `POST /users`

**权限要求**: 管理员

**请求头**:

- `Authorization: Bearer {access_token}`
- `Content-Type: application/json` (无文件) 或 `multipart/form-data` (有头像)

**请求参数** (JSON 格式):

```json
{
  "username": "new_student",
  "email": "new_student@example.com",
  "password": "temp_password123",
  "role": "student",
  "profile": {
    "name": "新学生",
    "student_id": "2025001",
    "class": "计算机科学2025级1班"
  }
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "用户创建成功",
  "data": {
    "id": 1002,
    "username": "new_student",
    "email": "new_student@example.com",
    "role": "student",
    "status": "active",
    "profile": {
      "name": "新学生",
      "student_id": "2025001",
      "class": "计算机科学2025级1班"
    },
    "created_at": "2025-06-13T15:30:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 4.3 更新用户信息

**接口地址**: `PUT /users/{user_id}`

**权限要求**: 管理员或用户本人

**请求头**: `Authorization: Bearer {access_token}`

**请求参数**:

```json
{
  "email": "updated_email@example.com",
  "role": "teacher",
  "status": "active",
  "profile": {
    "name": "更新后的姓名",
    "class": "计算机科学2025级2班"
  }
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "用户信息更新成功",
  "data": {
    "id": 1001,
    "username": "student001",
    "email": "updated_email@example.com",
    "role": "teacher",
    "status": "active",
    "updated_at": "2025-06-13T15:30:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 4.4 删除用户

**接口地址**: `DELETE /users/{user_id}`

**权限要求**: 管理员

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "用户删除成功",
  "timestamp": "2025-06-13T15:30:00Z"
}
```

---

## 5. 文件管理

### 5.1 上传文件

**接口地址**: `POST /files/upload`

**请求头**:

- `Authorization: Bearer {access_token}`
- `Content-Type: multipart/form-data`

**请求参数**:

- `file`: 文件内容 (必填)
- `type`: 文件类型 (homework_attachment, submission_attachment, avatar)
- `related_id`: 关联的作业或提交 ID (可选)

**响应示例**:

```json
{
  "code": 0,
  "message": "文件上传成功",
  "data": {
    "id": "file_abc123",
    "filename": "document.pdf",
    "original_name": "我的文档.pdf",
    "size": 1024000,
    "content_type": "application/pdf",
    "download_url": "/api/v1/files/download/abc123",
    "uploaded_at": "2025-06-13T15:30:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 5.2 下载文件

**接口地址**: `GET /files/download/{file_id}`

**请求头**: `Authorization: Bearer {access_token}`

**响应**: 文件流 (直接下载)

---

## 6. 系统管理

### 6.1 获取系统设置

**接口地址**: `GET /system/settings`

**权限要求**: 管理员

**请求头**: `Authorization: Bearer {access_token}`

**响应示例**:

```json
{
  "code": 0,
  "message": "获取系统设置成功",
  "data": {
    "max_file_size": 10485760,
    "allowed_file_types": [".pdf", ".docx", ".txt", ".py", ".java", ".cpp"],
    "max_attachments_per_submission": 5,
    "late_submission_penalty": 0.1,
    "auto_grade_enabled": false,
    "notification_enabled": true,
    "system_name": "作业管理系统",
    "maintenance_mode": false
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 6.2 更新系统设置

**接口地址**: `PUT /system/settings`

**权限要求**: 管理员

**请求头**: `Authorization: Bearer {access_token}`

**请求参数**:

```json
{
  "max_file_size": 20971520,
  "allowed_file_types": [
    ".pdf",
    ".docx",
    ".txt",
    ".py",
    ".java",
    ".cpp",
    ".zip"
  ],
  "max_attachments_per_submission": 10,
  "late_submission_penalty": 0.05,
  "notification_enabled": true
}
```

**响应示例**:

```json
{
  "code": 0,
  "message": "系统设置更新成功",
  "data": {
    "updated_settings": [
      "max_file_size",
      "allowed_file_types",
      "max_attachments_per_submission",
      "late_submission_penalty"
    ],
    "updated_at": "2025-06-13T15:30:00Z"
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

### 6.3 获取系统日志

**接口地址**: `GET /system/logs`

**权限要求**: 管理员

**请求头**: `Authorization: Bearer {access_token}`

**查询参数**:

- `page`: 页码 (默认: 1)
- `size`: 每页数量 (默认: 20)
- `level`: 日志级别 (info, warn, error)
- `start_date`: 开始日期 (ISO 8601 格式)
- `end_date`: 结束日期 (ISO 8601 格式)
- `module`: 模块名称 (auth, homework, submission, user)

**响应示例**:

```json
{
  "code": 0,
  "message": "获取系统日志成功",
  "data": {
    "items": [
      {
        "id": "log_001",
        "level": "info",
        "message": "用户 student001 登录成功",
        "module": "auth",
        "user_id": 1001,
        "ip_address": "192.168.1.100",
        "user_agent": "Mozilla/5.0...",
        "created_at": "2025-06-13T15:30:00Z"
      },
      {
        "id": "log_002",
        "level": "warn",
        "message": "文件上传大小超过限制",
        "module": "file",
        "user_id": 1002,
        "details": {
          "file_size": 15728640,
          "max_size": 10485760
        },
        "created_at": "2025-06-13T15:25:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "size": 20,
      "total": 150,
      "pages": 8
    }
  },
  "timestamp": "2025-06-13T15:30:00Z"
}
```

---

## 错误处理

### 错误响应格式

```json
{
  "code": 1001,
  "message": "请求参数错误",
  "errors": [
    {
      "field": "title",
      "message": "标题不能为空"
    }
  ],
  "timestamp": "2025-06-13T10:30:00Z"
}
```

---

## 附录

### A. 用户角色权限

| 角色   | 权限说明                           |
| ------ | ---------------------------------- |
| 学生   | 查看作业、提交作业、查看自己的成绩 |
| 教师   | 学生权限 + 创建作业、批改作业      |
| 管理员 | 所有权限 + 用户管理、系统设置      |

### B. 开发环境

```bash
# 启动开发服务器
cargo run

# 运行测试
cargo test

# 构建生产版本
cargo build --release
```

---

_文档版本: v1.0_  
_最后更新: 2025-06-13_
