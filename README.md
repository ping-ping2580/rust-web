# rust-web

一个基于 Rust 的练手项目，包含：

- `webservice`: 提供教师与课程相关的 REST API（Actix Web + SQLx + PostgreSQL）
- `webapp`: 服务端渲染页面（Tera 模板），通过 HTTP 调用 `webservice`
- `wasm-client`: Rust + WebAssembly 客户端示例

## 项目结构

```text
.
├── Cargo.toml              # workspace
├── .env                    # 环境变量（目前只配置了 DATABASE_URL）
├── webservice/             # 后端 API 服务
├── webapp/                 # 页面服务（SSR）
└── wasm-client/            # wasm 客户端
```

## 技术栈

- Rust 2024 / 2018（`wasm-client`）
- Actix Web
- SQLx
- PostgreSQL
- Tera
- wasm-bindgen / web-sys

## 环境准备

### 1) Rust 工具链

```bash
rustup show
cargo --version
```

### 2) PostgreSQL

本项目默认使用 PostgreSQL，请先保证数据库可访问。

`.env` 示例：

```env
DATABASE_URL=postgresql://postgres:123456@localhost:5432/postgres
# webapp 还需要：
HOST_PORT=127.0.0.1:8080
```

## 启动方式

建议分两个终端启动。

### 终端 A：启动 API（webservice）

```bash
cargo run -p db --bin teacher-service
```

默认监听：`127.0.0.1:3000`

健康检查：

```bash
curl http://127.0.0.1:3000/health
```

### 终端 B：启动页面服务（webapp）

```bash
export HOST_PORT=127.0.0.1:8080
cargo run -p webapp --bin svr
```

访问：`http://127.0.0.1:8080`

## 主要接口

### Teacher

- `GET /teachers/`
- `GET /teachers/{teacher_id}`
- `POST /teachers/`
- `PUT /teachers/{teacher_id}`
- `DELETE /teachers/{teacher_id}`

### Course

- `POST /courses/`
- `GET /courses/{teacher_id}`
- `GET /courses/{teacher_id}/{course_id}`
- `PUT /courses/{teacher_id}/{course_id}`
- `DELETE /courses/{teacher_id}/{course_id}`

## 本次修复说明

### 1) `webservice` 已迁移为 Axum

- 路由、handler、错误响应已从 Actix Web 风格切换为 Axum 风格
- 启动入口改为 `tokio + axum::serve`
- CORS 改为 `tower-http` 配置

### 2) teacher 相关 SQL 与类型映射已修复

- `Teacher` 增加 `sqlx::FromRow`，查询统一使用 `query_as`
- 修复 `id` 字段映射不一致导致的类型问题
- 删除接口改为参数绑定 SQL（避免字符串拼接），并在删除 0 行时返回 NotFound

### 3) `webapp` 字段命名与 API 对齐

- 前端表单字段统一为 `image_url`
- 调用 `webservice` 时映射为后端字段 `picture_url`
- 增加对 API 错误响应 `error_message` 的解析与页面回显

### 4) 编译状态

- `cargo check --workspace` 已可通过
- 当前仍有部分 warnings（主要在 `wasm-client`），不影响本次 fix

## 常用命令

```bash
# 检查整个 workspace
cargo check --workspace

# 单独检查可通过的包
cargo check -p webapp
cargo check -p wasm-client
```

## 后续建议

- 继续清理 `wasm-client` 的 warnings（unused import / deprecated API）
- 为数据库补充建表 SQL / migration
- 增加统一启动文档（`make`/`just`/脚本）
- 为关键 handler 和 dbaccess 增加集成测试
