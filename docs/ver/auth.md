---
doc_id: VER-AUTH
title: ACL验证用例
status: pending
owner: 产品负责人
version: 0.1
last-updated: 2026-11-02
---

# ACL验证用例

规则：状态只反映实际执行结果；未执行一律 `pending`，不得写成 `passed`。失败记录不得被覆盖。

CYCLE: 2026-11-02
范围：REQ-AUTH-ACL-002、REQ-AUTH-ACL-003、REQ-AUTH-ACL-004、REQ-AUTH-CRED-001、REQ-AUTH-CRED-002、REQ-AUTH-CRED-003、REQ-AUTH-CRED-004、REQ-AUTH-PRIV-001

VER-AUTH-ACL-003 status: pending
对应：REQ-AUTH-ACL-003
步骤：执行并观察「权限不足必须与连接故障、节点不存在区分显示，不得合并为同一种错误。」所述行为，记录实际结果。
结果：未执行

VER-AUTH-CRED-001 status: pending
对应：REQ-AUTH-CRED-001
步骤：执行并观察「凭据不得以明文持久化；无可用安全存储时不得降级为明文或写入普通配置文件。」所述行为，记录实际结果。
结果：未执行

VER-AUTH-CRED-002 status: pending
对应：REQ-AUTH-CRED-002
步骤：执行并观察「前端只接收非秘密的身份标识与展示名，秘密不得进入日志、错误、审计或脚本输出。」所述行为，记录实际结果。
结果：未执行

VER-AUTH-ACL-002 status: pending
对应：REQ-AUTH-ACL-002
步骤：打开有 ACL 的节点检查只读展示；再构造无读取权限的节点，确认「不可见」与「ACL 为空」显示不同。
结果：未执行

VER-AUTH-CRED-003 status: pending
对应：REQ-AUTH-CRED-003
步骤：新建凭证时检查默认保存范围为永不保存，并确认四档选项均可选。
结果：未执行

VER-AUTH-CRED-004 status: pending
对应：REQ-AUTH-CRED-004
步骤：分别以四种范围保存凭据，检查磁盘无明文；for session 在连接关闭后失效，until restart 在重启后失效。
结果：未执行

VER-AUTH-PRIV-001 status: pending
对应：REQ-AUTH-PRIV-001
步骤：抓包或检查进程外发请求，确认无遥测与崩溃上报，日志仅落本地。
结果：未执行

VER-AUTH-ACL-004 status: pending
对应：REQ-AUTH-ACL-004
步骤：用 setAcl 修改节点 ACL 与 -R 递归修改，确认确认框展示目标、身份与新 ACL；界面面板仍只读。
结果：未执行
