# 短链服务

![Test](https://github.com/ArronYR/short-url/blob/main/.github/workflows/rust.yml/badge.svg)

1. 支持生成短链接
2. 支持短链接访问

## Run

```shell
cargo run
```

## Deploy

项目的数据存储使用的MySQL数据库，在运行或部署时需要通过环境变量指定相关的配置信息：

```shell
PORT=80
ORIGIN=https://s.cn
DB_HOST=127.0.0.1
DB_PORT=3306
DB_USERNAME=short_url
DB_PASSWORD=******
RUST_LOG=info
CACHE_LIVE_TIME=7200
CACHE_MAX_CAP=1000
TOKEN=0uLcr3xYI2ndKTZv
```

- 如果不担心配置信息暴露，也可以直接修改代码中`init_config`默认的环境变量值。

## Usage

1. 生成短链接

```shell
# token 必须提供，用来控制非法调用，支持在环境变量中设置

curl 'http://s.cn/gen' \
-H 'token: 53ROYinHId9qke' \
-H 'Content-Type: application/json' \
-d '{
    "url": "https://www.baidu.com/s?ie=UTF-8&wd=test"
}'

```

2. 访问短链接

浏览器打开生成的短链接
