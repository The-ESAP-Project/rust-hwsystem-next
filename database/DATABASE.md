# 作业管理系统数据库设计文档

##### 当前版本
当前数据库版本：1

### 版本历史
| 版本 | 文件名 | 描述 |
|-----|--------|------|
| 1 | V1__init_migration_table.sql | 初始化迁移表 |

**注意**：由于项目尚未正式开始，除了初始迁移表外，其他所有表结构都已合并到主schema.sql文件中。这样可以更方便地进行初始化和调整。一旦项目正式启动并部署到生产环境后，所有的数据库变更都将通过迁移文件进行管理。
本项目使用基于文件的SQL迁移方案进行数据库版本管理。所有迁移文件位于 `database/migrations` 目录下。

### 迁移文件命名规则

迁移文件采用以下格式命名：

```
V{version}__{description}.sql
```

- `version`: 版本号，从1开始递增
- `description`: 迁移描述，使用下划线分隔单词

示例：`V1__init_migration_table.sql`

### 当前版本
当前数据库版本：4

### 版本历史
| 版本 | 文件名 | 描述 |
|-----|--------|------|
| 1 | V1__init_migration_table.sql | 初始化迁移表 |
| 2 | V2__create_base_tables.sql | 创建基础表结构 |
| 3 | V3__add_class_leader_role.sql | 添加课代表角色 |
| 4 | V4__add_permissions.sql | 添加权限管理系统 |

## 表结构设计

### users（用户表）
记录系统用户的基本信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| id | BIGSERIAL | 否 | | 主键 |
| username | VARCHAR(50) | 否 | | 用户名，唯一 |
| email | VARCHAR(255) | 否 | | 邮箱，唯一 |
| password_hash | VARCHAR(255) | 否 | | 密码哈希 |
| role | user_role | 否 | 'student' | 用户角色 |
| status | user_status | 否 | 'active' | 用户状态 |
| last_login | TIMESTAMP WITH TIME ZONE | 是 | | 最后登录时间 |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

**索引**：
- PRIMARY KEY (id)
- UNIQUE INDEX idx_users_username (username)
- UNIQUE INDEX idx_users_email (email)
- INDEX idx_users_role (role)

### user_profiles（用户配置文件表）
存储用户的详细信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| user_id | BIGINT | 否 | | 主键，关联users表 |
| name | VARCHAR(100) | 否 | | 真实姓名 |
| student_id | VARCHAR(50) | 是 | | 学号（仅学生） |
| class | VARCHAR(100) | 是 | | 班级 |
| avatar_url | VARCHAR(255) | 是 | | 头像URL |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

**约束**：
- PRIMARY KEY (user_id)
- FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
- CHECK ((role = 'student' AND student_id IS NOT NULL) OR (role != 'student'))

### homeworks（作业表）
存储作业信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| id | BIGSERIAL | 否 | | 主键 |
| title | VARCHAR(255) | 否 | | 作业标题 |
| description | TEXT | 是 | | 作业描述 |
| content | TEXT | 否 | | 作业内容 |
| deadline | TIMESTAMP WITH TIME ZONE | 否 | | 截止时间 |
| max_score | INTEGER | 否 | 100 | 最高分数 |
| allow_late_submission | BOOLEAN | 否 | false | 是否允许迟交 |
| status | homework_status | 否 | 'draft' | 作业状态 |
| created_by | BIGINT | 否 | | 创建者ID |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

**索引**：
- PRIMARY KEY (id)
- INDEX idx_homeworks_status (status)
- INDEX idx_homeworks_deadline (deadline)

### submissions（作业提交表）
记录学生提交的作业。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| id | BIGSERIAL | 否 | | 主键 |
| homework_id | BIGINT | 否 | | 作业ID |
| student_id | BIGINT | 否 | | 学生ID |
| content | TEXT | 否 | | 提交内容 |
| status | submission_status | 否 | 'submitted' | 提交状态 |
| score | INTEGER | 是 | | 分数 |
| is_late | BOOLEAN | 否 | false | 是否迟交 |
| submitted_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 提交时间 |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

**约束与索引**：
- PRIMARY KEY (id)
- FOREIGN KEY (homework_id) REFERENCES homeworks(id) ON DELETE CASCADE
- FOREIGN KEY (student_id) REFERENCES users(id)
- UNIQUE (homework_id, student_id)
- CHECK (score IS NULL OR (score >= 0 AND score <= 100))
- INDEX idx_submissions_homework_id (homework_id)
- INDEX idx_submissions_student_id (student_id)
- INDEX idx_submissions_status (status)

### feedbacks（批改反馈表）
存储教师对作业的批改信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| submission_id | BIGINT | 否 | | 主键，关联submissions表 |
| graded_by | BIGINT | 否 | | 批改教师ID |
| score | INTEGER | 否 | | 分数 |
| content | TEXT | 否 | | 反馈内容 |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

**约束**：
- PRIMARY KEY (submission_id)
- FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE
- FOREIGN KEY (graded_by) REFERENCES users(id)
- CHECK (score >= 0 AND score <= 100)

### files（文件表）
管理系统中的文件信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| id | UUID | 否 | uuid_generate_v4() | 主键 |
| filename | VARCHAR(255) | 否 | | 文件名 |
| original_name | VARCHAR(255) | 否 | | 原始文件名 |
| content_type | VARCHAR(100) | 否 | | 文件类型 |
| size | BIGINT | 否 | | 文件大小(字节) |
| type | file_type | 否 | | 文件用途类型 |
| related_id | BIGINT | 是 | | 关联ID |
| uploaded_by | BIGINT | 否 | | 上传者ID |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |

**索引**：
- PRIMARY KEY (id)
- INDEX idx_files_type (type)
- INDEX idx_files_related_id (related_id)

### system_settings（系统设置表）
存储系统配置信息。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| key | VARCHAR(50) | 否 | | 主键，设置项键名 |
| value | JSONB | 否 | | 设置值 |
| description | TEXT | 是 | | 设置项说明 |
| updated_by | BIGINT | 是 | | 更新者ID |
| updated_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 更新时间 |

### system_logs（系统日志表）
记录系统操作日志。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| id | BIGSERIAL | 否 | | 主键 |
| level | log_level | 否 | | 日志级别 |
| message | TEXT | 否 | | 日志消息 |
| module | VARCHAR(50) | 否 | | 模块名称 |
| user_id | BIGINT | 是 | | 相关用户ID |
| ip_address | INET | 是 | | IP地址 |
| user_agent | TEXT | 是 | | 用户代理 |
| details | JSONB | 是 | | 详细信息 |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |

**索引**：
- PRIMARY KEY (id)
- INDEX idx_system_logs_level (level)
- INDEX idx_system_logs_created_at (created_at)

## 枚举类型

### user_role（用户角色）
用户角色：
- student: 学生
- teacher: 教师
- admin: 管理员
- class_leader: 课代表

### user_status
用户状态：
- active: 活跃
- inactive: 未激活
- suspended: 已停用

### homework_status
作业状态：
- draft: 草稿
- published: 已发布
- closed: 已关闭

### submission_status
提交状态：
- submitted: 已提交
- graded: 已批改
- late: 迟交

### file_type
文件类型：
- homework_attachment: 作业附件
- submission_attachment: 提交附件
- avatar: 头像

### log_level
日志级别：
- info: 信息
- warn: 警告
- error: 错误

### permission_type（权限类型）
权限类型：
- view_homework: 查看作业
- submit_homework: 提交作业
- create_homework: 创建作业
- edit_homework: 编辑作业
- delete_homework: 删除作业
- grade_homework: 评分作业
- view_submissions: 查看提交
- manage_submissions: 管理提交
- manage_students: 管理学生
- manage_class: 管理班级
- system_admin: 系统管理

## 自动更新机制

系统使用触发器自动维护 `updated_at` 字段：

```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';
```

应用于以下表：
- users
- user_profiles
- homeworks
- submissions
- feedbacks

## 权限系统设计

### role_permissions（角色权限表）
定义每个角色拥有的权限。

| 字段 | 类型 | 可空 | 默认值 | 说明 |
|------|------|------|--------|------|
| role | user_role | 否 | | 用户角色 |
| permission | permission_type | 否 | | 权限类型 |
| created_at | TIMESTAMP WITH TIME ZONE | 否 | CURRENT_TIMESTAMP | 创建时间 |

**约束**：
- PRIMARY KEY (role, permission)

### 各角色默认权限

#### 学生（student）权限
- view_homework：查看作业
- submit_homework：提交作业
- view_submissions：查看自己的提交

#### 课代表（class_leader）权限
- view_homework：查看作业
- submit_homework：提交作业
- view_submissions：查看提交
- manage_submissions：管理提交（如协助检查）
- manage_class：管理班级（如协助收集作业）

#### 教师（teacher）权限
- view_homework：查看作业
- create_homework：创建作业
- edit_homework：编辑作业
- delete_homework：删除作业
- grade_homework：评分作业
- view_submissions：查看提交
- manage_submissions：管理提交
- manage_students：管理学生
- manage_class：管理班级

#### 管理员（admin）权限
拥有所有权限：
- view_homework：查看作业
- create_homework：创建作业
- edit_homework：编辑作业
- delete_homework：删除作业
- grade_homework：评分作业
- view_submissions：查看提交
- manage_submissions：管理提交
- manage_students：管理学生
- manage_class：管理班级
- system_admin：系统管理

### user_permissions（用户权限视图）
用于快速查询用户拥有的所有权限。

```sql
CREATE VIEW user_permissions AS
SELECT u.id as user_id, u.username, u.role, rp.permission
FROM users u
JOIN role_permissions rp ON u.role = rp.role;
```

## 表结构设计
