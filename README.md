Node.js 版本问题
Node.js v22.x 默认启用了更严格的 OpenSSL 配置（例如禁用了 MD4、MD5 等旧算法），而 Webpack 的某些版本（尤其是较旧的配置）可能仍依赖这些算法，导致 digital envelope routines::unsupported 错误。
Webpack 配置 错误堆栈显示问题出在 webpack/lib/util/createHash.js，表明 Webpack 在生成构建哈希时使用了不受支持的加密算法。
在 ws/wasm-client/www 目录下运行构建，可能是使用 create-wasm-app 的默认 Webpack 配置，针对 WebAssembly 项目。

Teacher表的id字段存在问题

