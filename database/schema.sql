-- 作业管理系统数据库表结构
-- PostgreSQL 版本

-- 启用 UUID 扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 创建枚举类型
CREATE TYPE user_role AS ENUM ('admin', 'teacher', 'student', 'class_leader');
CREATE TYPE user_status AS ENUM ('active', 'inactive', 'suspended');
CREATE TYPE homework_status AS ENUM ('draft', 'published', 'closed');
CREATE TYPE submission_status AS ENUM ('submitted', 'graded', 'late');
CREATE TYPE file_type AS ENUM ('homework_attachment', 'submission_attachment', 'avatar');
CREATE TYPE log_level AS ENUM ('info', 'warn', 'error');
CREATE TYPE permission_type AS ENUM (
    'view_homework',         -- 查看作业
    'submit_homework',       -- 提交作业
    'create_homework',       -- 创建作业
    'edit_homework',         -- 编辑作业
    'delete_homework',       -- 删除作业
    'grade_homework',        -- 评分作业
    'view_submissions',      -- 查看提交
    'manage_submissions',    -- 管理提交
    'manage_students',       -- 管理学生
    'manage_class',         -- 管理班级
    'system_admin'          -- 系统管理
);

-- 用户表
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'student',
    status user_status NOT NULL DEFAULT 'active',
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 用户配置文件表
CREATE TABLE user_profiles (
    user_id BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    student_id VARCHAR(50),
    class VARCHAR(100),
    avatar_url VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT chk_student_id CHECK (
        (role = 'student' AND student_id IS NOT NULL) OR
        (role != 'student')
    )
);

-- 作业表
CREATE TABLE homeworks (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    content TEXT NOT NULL,
    deadline TIMESTAMP WITH TIME ZONE NOT NULL,
    max_score INTEGER NOT NULL DEFAULT 100,
    allow_late_submission BOOLEAN NOT NULL DEFAULT false,
    status homework_status NOT NULL DEFAULT 'draft',
    created_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 作业提交表
CREATE TABLE submissions (
    id BIGSERIAL PRIMARY KEY,
    homework_id BIGINT NOT NULL REFERENCES homeworks(id) ON DELETE CASCADE,
    student_id BIGINT NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    status submission_status NOT NULL DEFAULT 'submitted',
    score INTEGER,
    is_late BOOLEAN NOT NULL DEFAULT false,
    submitted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_score CHECK (score IS NULL OR (score >= 0 AND score <= 100)),
    UNIQUE (homework_id, student_id)
);

-- 批改反馈表
CREATE TABLE feedbacks (
    submission_id BIGINT PRIMARY KEY REFERENCES submissions(id) ON DELETE CASCADE,
    graded_by BIGINT NOT NULL REFERENCES users(id),
    score INTEGER NOT NULL CHECK (score >= 0 AND score <= 100),
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 文件表
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename VARCHAR(255) NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    size BIGINT NOT NULL,
    type file_type NOT NULL,
    related_id BIGINT,
    uploaded_by BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 系统设置表
CREATE TABLE system_settings (
    key VARCHAR(50) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    updated_by BIGINT REFERENCES users(id),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 角色权限表
CREATE TABLE role_permissions (
    role user_role NOT NULL,
    permission permission_type NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role, permission)
);

-- 系统日志表
CREATE TABLE system_logs (
    id BIGSERIAL PRIMARY KEY,
    level log_level NOT NULL,
    message TEXT NOT NULL,
    module VARCHAR(50) NOT NULL,
    user_id BIGINT REFERENCES users(id),
    ip_address INET,
    user_agent TEXT,
    details JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 索引
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_homeworks_status ON homeworks(status);
CREATE INDEX idx_homeworks_deadline ON homeworks(deadline);
CREATE INDEX idx_submissions_homework_id ON submissions(homework_id);
CREATE INDEX idx_submissions_student_id ON submissions(student_id);
CREATE INDEX idx_submissions_status ON submissions(status);
CREATE INDEX idx_files_type ON files(type);
CREATE INDEX idx_files_related_id ON files(related_id);
CREATE INDEX idx_system_logs_level ON system_logs(level);
CREATE INDEX idx_system_logs_created_at ON system_logs(created_at);

-- 触发器：自动更新 updated_at 字段
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_profiles_updated_at
    BEFORE UPDATE ON user_profiles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_homeworks_updated_at
    BEFORE UPDATE ON homeworks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_submissions_updated_at
    BEFORE UPDATE ON submissions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_feedbacks_updated_at
    BEFORE UPDATE ON feedbacks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- 默认系统设置
INSERT INTO system_settings (key, value, description) VALUES
('max_file_size', '10485760'::jsonb, '最大文件大小（字节）'),
('allowed_file_types', '["pdf", "docx", "txt", "py", "java", "cpp"]'::jsonb, '允许的文件类型'),
('max_attachments_per_submission', '5'::jsonb, '每次提交最大附件数'),
('late_submission_penalty', '0.1'::jsonb, '迟交作业扣分比例'),
('notification_enabled', 'true'::jsonb, '是否启用通知'),
('system_name', '"作业管理系统"'::jsonb, '系统名称'),
('maintenance_mode', 'false'::jsonb, '维护模式');

-- 插入基础权限配置
-- 学生权限
INSERT INTO role_permissions (role, permission) VALUES
    ('student', 'view_homework'),
    ('student', 'submit_homework'),
    ('student', 'view_submissions');

-- 课代表权限
INSERT INTO role_permissions (role, permission) VALUES
    ('class_leader', 'view_homework'),
    ('class_leader', 'submit_homework'),
    ('class_leader', 'view_submissions'),
    ('class_leader', 'manage_submissions'),
    ('class_leader', 'manage_class');

-- 教师权限
INSERT INTO role_permissions (role, permission) VALUES
    ('teacher', 'view_homework'),
    ('teacher', 'create_homework'),
    ('teacher', 'edit_homework'),
    ('teacher', 'delete_homework'),
    ('teacher', 'grade_homework'),
    ('teacher', 'view_submissions'),
    ('teacher', 'manage_submissions'),
    ('teacher', 'manage_students'),
    ('teacher', 'manage_class');

-- 管理员权限
INSERT INTO role_permissions (role, permission) VALUES
    ('admin', 'view_homework'),
    ('admin', 'create_homework'),
    ('admin', 'edit_homework'),
    ('admin', 'delete_homework'),
    ('admin', 'grade_homework'),
    ('admin', 'view_submissions'),
    ('admin', 'manage_submissions'),
    ('admin', 'manage_students'),
    ('admin', 'manage_class'),
    ('admin', 'system_admin');

-- 创建视图用于查询角色权限
CREATE VIEW user_permissions AS
SELECT u.id as user_id, u.username, u.role, rp.permission
FROM users u
JOIN role_permissions rp ON u.role = rp.role;

-- 注释
COMMENT ON TABLE users IS '用户表';
COMMENT ON TABLE user_profiles IS '用户配置文件表';
COMMENT ON TABLE homeworks IS '作业表';
COMMENT ON TABLE submissions IS '作业提交表';
COMMENT ON TABLE feedbacks IS '批改反馈表';
COMMENT ON TABLE files IS '文件表';
COMMENT ON TABLE system_settings IS '系统设置表';
COMMENT ON TABLE system_logs IS '系统日志表';
