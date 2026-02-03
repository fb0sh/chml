# chml

一个用于 [Chmlfrp](https://www.chmlfrp.net) 的命令行工具，让你可以轻松管理隧道、域名和节点。

## 功能特性

- 🚀 快速创建和管理隧道（TCP/UDP/HTTP）
- 📋 列出隧道、域名和节点信息
- 🔗 一键连接隧道
- ⚙️ 获取隧道配置文件
- 🎯 快捷命令：快速创建常见类型的隧道

## 安装

### 使用 Cargo 安装

确保你已经安装了 [Rust](https://www.rust-lang.org/tools/install)，然后运行：

```bash
cargo install chml
```

### 从源码安装

```bash
git clone https://github.com/fb0sh/chml.git
cd chml
cargo install --path .
```

## 配置

在使用之前，需要设置环境变量：

```bash
export CHML_API_BASE_URL="http://cf-v2.uapis.cn"
export CHML_API_TOKEN="YkJ31tP6Ev4HCSlP2D6Ifc0e"
```

你可以将这些环境变量添加到你的 shell 配置文件中（如 `~/.zshrc` 或 `~/.bashrc`）。

## 使用方法

### 查看帮助

```bash
chml --help
```

<img width="670" height="562" alt="image" src="https://github.com/user-attachments/assets/ae8d6e19-1139-4583-be83-21891a83de6a" />


### 列出资源

列出所有隧道和域名（默认）：

```bash
chml ls
```

<img width="1301" height="557" alt="image" src="https://github.com/user-attachments/assets/a4427558-06d1-46f9-9a4e-214b0c368715" />

仅列出隧道：

```bash
chml ls -t
```

仅列出域名：

```bash
chml ls -d
```

仅列出节点：

```bash
chml ls -n
```
<img width="1255" height="782" alt="image" src="https://github.com/user-attachments/assets/800f6cbc-9728-459c-95d8-9ca3ebc887ad" />

列出所有配置文件：

```bash
chml ls -c
```
<img width="1056" height="131" alt="image" src="https://github.com/user-attachments/assets/0407a5f8-23f0-4380-b1e6-e653170c94b1" />

列出全部
```bash
chml ls -tdnc
```

### 快速创建隧道

创建 TCP 隧道：

```bash
chml tcp 4444
```
<img width="1327" height="295" alt="image" src="https://github.com/user-attachments/assets/b9a24024-c34e-4d01-ad04-1983b9e2b8a9" />

<img width="419" height="273" alt="image" src="https://github.com/user-attachments/assets/826703b0-de26-44c3-92fb-0ecc6f1bec27" />

会自动进行删除隧道和配置文件

<img width="1277" height="357" alt="image" src="https://github.com/user-attachments/assets/c9a9d387-1464-4559-ad49-26aaf9674d25" />

创建 UDP 隧道：

```bash
chml udp 4444
```

创建 HTTP 隧道：（直接使用tcp）

```bash
chml http 8080
```
### 添加隧道

添加一个 TCP 隧道：

```bash
chml add tunnel --type tcp --lport 4444 --name my-tunnel
```

<img width="1191" height="215" alt="image" src="https://github.com/user-attachments/assets/66ac13f6-ddac-4edf-893d-b0a4a7eb7c43" />


添加一个 HTTP 隧道：

```bash
chml add tunnel --type http --lport 8080 --name my-http
```

指定节点和远程端口：

```bash
chml add tunnel --type tcp --lport 4444 --node "节点名称" --rport 88888
```

指定本地主机：

```bash
chml add tunnel --type tcp --lport 4444 --lhost 127.0.0.1
```


### 连接隧道

通过隧道名称连接：

```bash
chml connect -t my-tunnel
```
<img width="1226" height="264" alt="image" src="https://github.com/user-attachments/assets/2b6d0b62-2ba2-4942-b542-d9425aa5c35d" />


通过隧道 ID 连接：

```bash
chml connect -i 12345
```

后台运行（守护进程模式）：(暂不支持)

```bash
chml connect -t my-tunnel --daemon
```

### 获取隧道配置

获取指定隧道的配置：

```bash
chml get -t my-tunnel
```

<img width="1025" height="344" alt="image" src="https://github.com/user-attachments/assets/d196e487-c550-4039-b097-9837f867829c" />


### 删除隧道

通过隧道名称删除：

```bash
chml rm -t my-tunnel
```

通过隧道 ID 删除：

```bash
chml rm --tunnel-id 12345
```

### 静默模式

所有命令都支持 `-q/--quiet` 选项，用于减少输出信息：

```bash
chml -q ls
chml -q connect -t my-tunnel
```

## 命令速查表

| 命令 | 说明 |
|------|------|
| `chml ls` | 列出隧道和域名 |
| `chml ls -t` | 仅列出隧道 |
| `chml ls -d` | 仅列出域名 |
| `chml ls -n` | 仅列出节点 |
| `chml ls -c` | 列出配置文件 |
| `chml tcp <port>` | 快速创建 TCP 隧道 |
| `chml udp <port>` | 快速创建 UDP 隧道 |
| `chml http <port>` | 快速创建 HTTP 隧道 |
| `chml add tunnel` | 添加隧道 |
| `chml connect -t <name>` | 连接隧道 |
| `chml get -t <name>` | 获取隧道配置 |
| `chml rm -t <name>` | 删除隧道 |

## 工作目录

`chml` 会在你的系统目录下创建工作目录：

- **macOS/Linux**: `~/.chml/`
- **Windows**: `%USERPROFILE%\.chml\`

目录结构：
```
~/.chml/
├── bin/          # frpc 二进制文件
└── conf/         # frpc 配置文件
```

## 开发

```bash
# 克隆仓库
git clone https://github.com/fb0sh/chml.git
cd chml

# 运行
cargo run -- --help

# 构建
cargo build --release
```

## 许可证

MIT License

## 相关链接

- [Chmlfrp 官网](https://www.chmlfrp.net)
- [Chmlfrp 面板](https://panel.chmlfrp.net)
- [GitHub 仓库](https://github.com/fb0sh/chml)

## 贡献

欢迎提交 Issue 和 Pull Request！
