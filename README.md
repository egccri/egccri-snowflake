# Egccri-Snowflake
> 支持自定义单节点snowflake生成策略，支持美团leaf生成策略

#### QuickStart

1. 本地启动zookeeper节点

2. 然后启动main服务

3. 运行example里的`grpc_client`

#### Features

- [x] custom snowflake
- [x] leaf snowflake
- [x] grpc endpoint
- [ ] http endpoint
- [ ] 人体工学配置
- [ ] 单请求多id生成

#### Zookeeper 节点信息

```
/snowflake/com.egccri.meta/forever
/snowflake/com.egccri.meta/forever/127.0.0.1:50051-0000000000
/snowflake/com.egccri.meta/forever/127.0.0.1:50052-0000000001
```

#### 链接

https://tech.meituan.com/2017/04/21/mt-leaf.html
