# 短链服务

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

> request
curl -L 'http://127.0.0.1/api/generate' \
-H 'Token: 53ROYinHId9qke' \
-H 'Content-Type: application/json' \
-d '{
    "urls": [
        "https://dtsnew.console.aliyun.com/connect/cn-beijing",
        "https://dtsnew.console.aliyun.com/connect/cn-hangzhou",
        "https://dtsnew.console.aliyun.com/connect/cn-zhangjiakou",
        "https://dtsnew.console.aliyun.com/connect/cn-shanghai"
    ]
}'

> response
{
    "f3b5941fb8a51565526a5353242ab0d4": "https://127.0.0.1/mWejF",
    "4bce96459bcb6c6dc8e3428bc6dbec98": "https://127.0.0.1/WFu7j",
    "35798d470756c2f87a10fc884d9df82d": "https://127.0.0.1/d5GHE",
    "b4962b8b32c1684aa7eadd0b3cdc93ab": "https://127.0.0.1/KsQsa"
}
```

2. 访问短链接

浏览器打开生成的短链接
