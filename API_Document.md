# API Documentation

## 1. Authentication

### Login
- **Endpoint:** `/api/v1/auth/login`
- **Method:** POST
- **Description:** 用户登录
- **Request Body:**
```json
{
  "username": "student123",
  "password": "hashed_password123"
}
```
- **Response:**
```json
{
  "code": 0,
  "msg": "登录成功",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIs...",
    "user": {
      "id": 1001,
      "username": "student123",
      "role": "student"
    }
  }
}
```

## 2. Homework Management

### Create Homework
- **Endpoint:** `/api/v1/homework/create`
- **Method:** POST
- **Description:** 创建新作业
- **Request Body:**
```json
{
  "title": "数学作业",
  "content": "完成习题1-10",
  "deadline": "2023-12-31 23:59"
}
```

### List Homework
- **Endpoint:** `/api/v1/homework/list`
- **Method:** GET
- **Description:** 列出所有作业
- **Query Params:** `page=1&size=10`

### View Homework
- **Endpoint:** `/api/v1/homework/view`
- **Method:** GET
- **Description:** 查看作业详情
- **Query Params:** `homework_id=1001`

### Modify Homework
- **Endpoint:** `/api/v1/homework/modify`
- **Method:** PUT
- **Description:** 修改作业
- **Request Body:**
```json
{
  "homework_id": 1001,
  "title": "更新后的标题",
  "content": "更新后的内容"
}
```

### Delete Homework
- **Endpoint:** `/api/v1/homework/delete`
- **Method:** DELETE
- **Description:** 删除作业
- **Request Body:**
```json
{
  "homework_id": 1001
}
```

### Submit Homework
- **Endpoint:** `/api/v1/homework/submit`
- **Method:** POST
- **Description:** 提交作业
- **Request Body:**
```json
{
  "homework_id": 1001,
  "content": "作业内容...",
  "attachments": ["file1.pdf", "file2.jpg"]
}
```

### Reply to Homework
- **Endpoint:** `/api/v1/homework/reply`
- **Method:** POST
- **Description:** 批改作业
- **Request Body:**
```json
{
  "submission_id": 5001,
  "score": 95,
  "feedback": "完成得很好！"
}
```

## 3. Submission Management

### List Submissions by Time
- **Endpoint:** `/api/v1/submission/list/time`
- **Method:** GET
- **Description:** 按时间排序提交
- **Query Params:** `homework_id=1001&order=desc`

### List All Submissions
- **Endpoint:** `/api/v1/submission/list/all`
- **Method:** GET
- **Description:** 按学生姓名排序提交
- **Query Params:** `homework_id=1001`

### Export Submissions
- **Endpoint:** `/api/v1/submission/export`
- **Method:** POST
- **Description:** 导出提交统计
- **Request Body:**
```json
{
  "homework_id": 1001,
  "export_type": "excel"
}
```

## 4. System Management

### View Logs
- **Endpoint:** `/api/v1/logs/view`
- **Method:** GET
- **Description:** 查看系统日志
- **Query Params:** `type=error&date=2023-10-01`

### Export Logs
- **Endpoint:** `/api/v1/logs/export`
- **Method:** GET
- **Description:** 导出系统日志

### Modify System Settings
- **Endpoint:** `/api/v1/system/modify`
- **Method:** PUT
- **Description:** 修改系统设置
- **Request Body:**
```json
{
  "max_file_size": 10,
  "allowed_types": [".pdf", ".docx"],
  "max_attachments": 3
}
```

## 5. User Management

### List Users
- **Endpoint:** `/api/v1/users/list`
- **Method:** GET
- **Description:** 列出所有用户
- **Query Params:** `role=student`

### Add User
- **Endpoint:** `/api/v1/users/add`
- **Method:** POST
- **Description:** 添加用户
- **Request Body:**
```json
{
  "username": "new_user",
  "password": "init_password",
  "role": "teacher",
  "email": "user@example.com"
}
```

### Delete User
- **Endpoint:** `/api/v1/users/delete`
- **Method:** DELETE
- **Description:** 删除用户
- **Request Body:**
```json
{
  "user_id": 1002
}
```

### Modify User
- **Endpoint:** `/api/v1/users/modify`
- **Method:** PUT
- **Description:** 修改用户信息
- **Request Body:**
```json
{
  "user_id": 1002,
  "role": "admin",
  "status": "active"
}
```
