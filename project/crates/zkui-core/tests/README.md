# 客户端对照测试（Spike）

按库拆分目录，避免两套客户端混在一个文件里。每个库一个测试二进制，内部再按关注点分模块。

```text
tests/
  zookeeper_client.rs        zookeeper-client 入口
  zookeeper_client/          common crud acl watch errors concurrency multi limits cleanup
  zookeeper_async.rs         zookeeper-async 入口
  zookeeper_async/           common crud acl watch errors limits
```

## 运行

测试默认 `#[ignore]`，需要真实 ZooKeeper 环境。先启动场景，再显式运行：

```bash
just --justfile test/env/justfile up zk-39-single

CARGO_HOME=$PWD/.cargo-home ZK_SPIKE_ENDPOINT=127.0.0.1:2181 \
cargo test -p zkui-core -- --ignored --test-threads=1

just --justfile test/env/justfile down zk-39-single
```

只跑某一个库：

```bash
cargo test -p zkui-core --test zookeeper_client -- --ignored --test-threads=1
cargo test -p zkui-core --test zookeeper_async  -- --ignored --test-threads=1
```

## 约定

- 用例使用根下 `zkc-` / `zka-` 前缀的唯一路径，结束时递归清理；`cleanup` 模块兜底检查。
- 失败时记录错误类别，不吞错；不假设服务端四字命令可用。
- 结果写入 `docs/ver/` 或对应 CHG，不在对话里口述结论。
