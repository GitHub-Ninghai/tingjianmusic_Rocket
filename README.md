# 项目搭建

[![Rocket Homepage](https://img.shields.io/badge/web-rocket.rs-red.svg?style=flat&label=https&colorB=d33847)](https://rocket.rs)

[![Current Crates.io Version](https://img.shields.io/crates/v/rocket.svg)](https://crates.io/crates/rocket)

本项目使用Rust语言

前端：:tangerine:Yew 

后端：🚀Rocket

数据库：:dolphin:Mysql

缓存：:shopping_cart: Redis

## 后端搭建

通过终端打开music目录

安装连接mysql数据库所需的diesel

`cargo install diesel_cli --no-default-features --features mysql`

env文件中的

```
DATABASE_URL=mysql://[ninghai[:Yyt00200059]@]uf6q69f2nvrhjkbkymysqlpolardbreindeer.mysql.polardb.rds.aliyuncs.com/Hearingmusic
```

使用命令`cargo run`运行后端项目

## 前端搭建

##### 安装 WebAssembly 目标

`rustup target add wasm32-unknown-unknown`

安装中继

```
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

`cargo install trunk`

创建过程：

`cargo new music-app`

`cd music-app`

使用一下命令在本地生成和提供服务应用程序

`trunk serve`

#### 使用 wasm-pack

```rust
[lib]
crate-type = ["rlib", "cdylib"]
```

`cargo install wasm-pack`



