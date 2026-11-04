//! `no_proxy` 匹配：精确主机与域名后缀。

/// 判断主机是否命中绕过列表。
///
/// 规则（大小写不敏感）：
/// - 精确匹配：`zk.internal`；
/// - 后缀匹配：`.corp.example` 或 `*.corp.example`，命中 `a.corp.example`；
/// - `*` 表示全部绕过。
pub fn matches(host: &str, patterns: &[String]) -> bool {
    let host = host.trim().to_ascii_lowercase();
    patterns.iter().any(|pattern| {
        let pattern = pattern.trim().to_ascii_lowercase();
        if pattern.is_empty() {
            return false;
        }
        if pattern == "*" {
            return true;
        }
        let (pattern, suffix_mode) = if let Some(rest) = pattern.strip_prefix("*.") {
            (rest, true)
        } else if let Some(rest) = pattern.strip_prefix('.') {
            (rest, true)
        } else {
            (pattern.as_str(), false)
        };
        if suffix_mode {
            host == pattern || host.ends_with(&format!(".{pattern}"))
        } else {
            host == pattern
        }
    })
}

#[cfg(test)]
mod tests {
    use super::matches;

    fn list(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn exact_host_matches() {
        assert!(matches("zk.internal", &list(&["zk.internal"])));
        assert!(!matches("zk.internal", &list(&["other.internal"])));
    }

    #[test]
    fn suffix_matches_subdomains() {
        let patterns = list(&[".corp.example"]);
        assert!(matches("a.corp.example", &patterns));
        assert!(matches("corp.example", &patterns));
        assert!(!matches("evilcorp.example", &patterns));
    }

    #[test]
    fn wildcard_suffix_matches() {
        assert!(matches("a.corp.example", &list(&["*.corp.example"])));
    }

    #[test]
    fn wildcard_matches_everything() {
        assert!(matches("anything", &list(&["*"])));
    }

    #[test]
    fn matching_is_case_insensitive() {
        assert!(matches("ZK.Internal", &list(&["zk.internal"])));
    }
}
