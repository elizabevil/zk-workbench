# 仓库统一入口：只负责编排，具体检查在各自的 justfile。
import 'deploy/dev.justfile'

docs-check:
    just --justfile docs/justfile check

code-check:
    just --justfile project/justfile check

check: docs-check code-check
