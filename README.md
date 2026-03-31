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

## 当前已知问题

### 1) `webservice` 编译失败（阻塞问题）

`teacher` 模型里 `id: i32`，但部分 SQLx 查询返回推断为 `Option<i32>`，存在类型不匹配，导致 `cargo check --workspace` 失败。

涉及文件：

- `webservice/src/models/teacher.rs`
- `webservice/src/dbaccess/teacher.rs`

### 2) SQLx 编译期依赖数据库

项目中使用了 `sqlx::query!` 宏。该宏默认会在编译期校验 SQL，需要数据库可访问，或提供离线元数据。

如果数据库不可访问，编译时会出现类似：

```text
error communicating with database
```

### 3) `webapp` 依赖 `HOST_PORT`

`webapp/src/bin/svr.rs` 会读取 `HOST_PORT`。未设置时启动会直接报错退出。

### 4) wasm-client 的 Node.js 版本兼容问题

在 `wasm-client/www` 使用较旧 Webpack 构建时，Node.js 22 可能出现 OpenSSL 相关错误：

```text
digital envelope routines::unsupported
```

可选处理方式：

- 使用较低 Node LTS（如 Node 18/20）
- 或升级 webpack 配置与相关依赖

## 常用命令

```bash
# 检查整个 workspace（当前会因 webservice 问题失败）
cargo check --workspace

# 单独检查可通过的包
cargo check -p webapp
cargo check -p wasm-client
```

## 后续建议

- 先修复 `teacher.id` 类型与 SQL 查询映射，确保 `webservice` 可编译可运行
- 为数据库补充建表 SQL / migration
- 增加统一启动文档（`make`/`just`/脚本）
- 为关键 handler 和 dbaccess 增加集成测试
