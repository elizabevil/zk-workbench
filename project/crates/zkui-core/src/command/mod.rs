//! zkCli 风格命令：解析与执行。
//!
//! 首版命令集见 `REQ-CLI-SCOPE-006`；watcher 开关由界面承担，`-w` 明确报错。

use zookeeper_client::{Acl, AuthId, CreateMode, Permission, Stat};

use crate::connection::{AuthScheme, ConnectionManager};
use crate::error::{AppResult, app};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Ls {
        path: String,
        recursive: bool,
        stat: bool,
    },
    Get {
        path: String,
        stat: bool,
    },
    Set {
        path: String,
        data: String,
        version: Option<i32>,
        stat: bool,
    },
    Create {
        path: String,
        data: String,
        sequential: bool,
        ephemeral: bool,
        container: bool,
    },
    Delete {
        path: String,
        version: Option<i32>,
    },
    DeleteAll {
        path: String,
    },
    Stat {
        path: String,
    },
    GetAcl {
        path: String,
        stat: bool,
    },
    SetAcl {
        path: String,
        acl: String,
        version: Option<i32>,
        recursive: bool,
    },
    AddAuth {
        scheme: String,
        auth: String,
    },
    Sync {
        path: String,
    },
    AllChildrenNumber {
        path: String,
    },
    Ephemerals {
        path: String,
    },
    ListQuota {
        path: String,
    },
    Version,
    Whoami,
    Help,
}

impl Command {
    /// 写操作返回确认摘要；只读命令返回 None。
    pub fn write_summary(&self) -> Option<String> {
        match self {
            Command::Set { path, .. } => Some(format!("set {path}")),
            Command::Create {
                path, ephemeral, ..
            } => Some(format!(
                "create {path}{}",
                if *ephemeral { " (ephemeral)" } else { "" }
            )),
            Command::Delete { path, .. } => Some(format!("delete {path}")),
            Command::DeleteAll { path } => Some(format!("deleteall {path}")),
            Command::SetAcl { path, .. } => Some(format!("setAcl {path}")),
            _ => None,
        }
    }
}

pub fn parse(line: &str) -> AppResult<Command> {
    let tokens = tokenize(line)?;
    if tokens.is_empty() {
        return Err(invalid("empty command"));
    }
    let name = tokens[0].to_ascii_lowercase();
    let args = &tokens[1..];
    match name.as_str() {
        "ls" => {
            let mut recursive = false;
            let mut stat = false;
            let mut path = None;
            for arg in args {
                match arg.as_str() {
                    "-R" => recursive = true,
                    "-s" => stat = true,
                    "-w" => {
                        return Err(invalid("watcher is managed by the UI; -w is not supported"));
                    }
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => path = Some(other.to_string()),
                }
            }
            Ok(Command::Ls {
                path: path.unwrap_or_else(|| "/".to_string()),
                recursive,
                stat,
            })
        }
        "get" => {
            let mut stat = false;
            let mut path = None;
            for arg in args {
                match arg.as_str() {
                    "-s" => stat = true,
                    "-w" => {
                        return Err(invalid("watcher is managed by the UI; -w is not supported"));
                    }
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => path = Some(other.to_string()),
                }
            }
            Ok(Command::Get {
                path: require(path)?,
                stat,
            })
        }
        "set" => {
            let mut stat = false;
            let mut version = None;
            let mut positional = Vec::new();
            let mut index = 0;
            while index < args.len() {
                match args[index].as_str() {
                    "-s" => stat = true,
                    "-v" => {
                        index += 1;
                        version = Some(parse_version(args.get(index))?);
                    }
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => positional.push(other.to_string()),
                }
                index += 1;
            }
            let path = positional
                .first()
                .cloned()
                .ok_or_else(|| invalid("set requires a path"))?;
            let data = positional.get(1).cloned().unwrap_or_default();
            Ok(Command::Set {
                path,
                data,
                version,
                stat,
            })
        }
        "create" => {
            let mut sequential = false;
            let mut ephemeral = false;
            let mut container = false;
            let mut positional = Vec::new();
            for arg in args {
                match arg.as_str() {
                    "-s" => sequential = true,
                    "-e" => ephemeral = true,
                    "-c" => container = true,
                    "-t" => return Err(invalid("-t ttl is not supported")),
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => positional.push(other.to_string()),
                }
            }
            let path = positional
                .first()
                .cloned()
                .ok_or_else(|| invalid("create requires a path"))?;
            let data = positional.get(1).cloned().unwrap_or_default();
            Ok(Command::Create {
                path,
                data,
                sequential,
                ephemeral,
                container,
            })
        }
        "delete" => {
            let mut version = None;
            let mut positional = Vec::new();
            let mut index = 0;
            while index < args.len() {
                match args[index].as_str() {
                    "-v" => {
                        index += 1;
                        version = Some(parse_version(args.get(index))?);
                    }
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => positional.push(other.to_string()),
                }
                index += 1;
            }
            let path = positional
                .first()
                .cloned()
                .ok_or_else(|| invalid("delete requires a path"))?;
            Ok(Command::Delete { path, version })
        }
        "deleteall" => Ok(Command::DeleteAll {
            path: require(args.first().cloned())?,
        }),
        "stat" => {
            let mut path = None;
            for arg in args {
                match arg.as_str() {
                    "-w" => {
                        return Err(invalid("watcher is managed by the UI; -w is not supported"));
                    }
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => path = Some(other.to_string()),
                }
            }
            Ok(Command::Stat {
                path: require(path)?,
            })
        }
        "getacl" => {
            let mut stat = false;
            let mut path = None;
            for arg in args {
                match arg.as_str() {
                    "-s" => stat = true,
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => path = Some(other.to_string()),
                }
            }
            Ok(Command::GetAcl {
                path: require(path)?,
                stat,
            })
        }
        "setacl" => {
            let mut version = None;
            let mut recursive = false;
            let mut positional = Vec::new();
            let mut index = 0;
            while index < args.len() {
                match args[index].as_str() {
                    "-R" => recursive = true,
                    "-v" => {
                        index += 1;
                        version = Some(parse_version(args.get(index))?);
                    }
                    "-s" => {}
                    other if other.starts_with('-') => {
                        return Err(invalid(&format!("unknown flag {other}")));
                    }
                    other => positional.push(other.to_string()),
                }
                index += 1;
            }
            let path = positional
                .first()
                .cloned()
                .ok_or_else(|| invalid("setAcl requires a path"))?;
            let acl = positional
                .get(1)
                .cloned()
                .ok_or_else(|| invalid("setAcl requires an ACL"))?;
            Ok(Command::SetAcl {
                path,
                acl,
                version,
                recursive,
            })
        }
        "addauth" => {
            let scheme = args
                .first()
                .cloned()
                .ok_or_else(|| invalid("addauth requires a scheme"))?;
            let auth = args
                .get(1)
                .cloned()
                .ok_or_else(|| invalid("addauth requires credentials"))?;
            Ok(Command::AddAuth { scheme, auth })
        }
        "sync" => Ok(Command::Sync {
            path: require(args.first().cloned())?,
        }),
        "getallchildrennumber" => Ok(Command::AllChildrenNumber {
            path: require(args.first().cloned())?,
        }),
        "getephemerals" => Ok(Command::Ephemerals {
            path: require(args.first().cloned())?,
        }),
        "listquota" => Ok(Command::ListQuota {
            path: require(args.first().cloned())?,
        }),
        "version" => Ok(Command::Version),
        "whoami" => Ok(Command::Whoami),
        "help" => Ok(Command::Help),
        other => Err(invalid(&format!("unsupported command: {other}"))),
    }
}

fn require(value: Option<String>) -> AppResult<String> {
    value.ok_or_else(|| invalid("missing path argument"))
}

fn parse_version(value: Option<&String>) -> AppResult<i32> {
    value
        .ok_or_else(|| invalid("-v requires a version"))?
        .parse()
        .map_err(|_| invalid("version must be an integer"))
}

fn tokenize(line: &str) -> AppResult<Vec<String>> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaped = false;
    for ch in line.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match quote {
            Some(q) => {
                if ch == '\\' {
                    escaped = true;
                } else if ch == q {
                    quote = None;
                } else {
                    current.push(ch);
                }
            }
            None => match ch {
                '\\' => escaped = true,
                '"' | '\'' => quote = Some(ch),
                c if c.is_whitespace() => {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                }
                c => current.push(c),
            },
        }
    }
    if quote.is_some() {
        return Err(invalid("unterminated quote"));
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn invalid(detail: &str) -> Box<crate::error::AppError> {
    Box::new(app::command_invalid(detail))
}

pub async fn execute(
    manager: &ConnectionManager,
    connection_id: &str,
    command: &Command,
) -> AppResult<String> {
    let client = manager.client_for_command(connection_id)?;
    match command {
        Command::Version => Ok(format!("ZK Workbench {}", env!("CARGO_PKG_VERSION"))),
        Command::Help => Ok(help_text()),
        Command::Whoami => {
            let users = client.list_auth_users().await?;
            if users.is_empty() {
                Ok("not authenticated".to_string())
            } else {
                Ok(users
                    .iter()
                    .map(|user| format!("{}:{}", user.scheme(), user.user()))
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
        }
        Command::AddAuth { scheme, auth } => {
            let scheme = AuthScheme::parse_scheme(scheme)
                .ok_or_else(|| invalid("unknown auth scheme (digest/sasl/ip/x509)"))?;
            if scheme != AuthScheme::Digest {
                return Err(invalid(
                    "only digest authentication is supported in this release",
                ));
            }
            client.auth(scheme.as_str(), auth.as_bytes()).await?;
            Ok("Authenticated".to_string())
        }
        Command::Sync { path } => {
            client.sync(path).await?;
            Ok("OK".to_string())
        }
        Command::Ls {
            path,
            recursive,
            stat,
        } => {
            let (children, node_stat) = client.get_children(path).await?;
            let mut output = String::new();
            if !recursive {
                output.push_str(&children.join("\n"));
            } else {
                let mut queue = std::collections::VecDeque::from([path.clone()]);
                while let Some(current) = queue.pop_front() {
                    let (names, _) = client.get_children(&current).await?;
                    for name in names {
                        let child = if current == "/" {
                            format!("/{name}")
                        } else {
                            format!("{current}/{name}")
                        };
                        output.push_str(&child);
                        output.push('\n');
                        queue.push_back(child);
                    }
                }
            }
            if *stat {
                output.push_str(&format_stat(path, &node_stat));
            }
            Ok(output)
        }
        Command::Get { path, stat } => {
            let (data, node_stat) = client.get_data(path).await?;
            let text = String::from_utf8_lossy(&data).to_string();
            let mut output = text;
            output.push('\n');
            if *stat {
                output.push_str(&format_stat(path, &node_stat));
            }
            Ok(output)
        }
        Command::Set {
            path,
            data,
            version,
            stat,
        } => {
            let node_stat = client.set_data(path, data.as_bytes(), *version).await?;
            let mut output = "OK".to_string();
            if *stat {
                output.push_str(&format_stat(path, &node_stat));
            }
            Ok(output)
        }
        Command::Create {
            path,
            data,
            sequential,
            ephemeral,
            container,
        } => {
            use zookeeper_client::Acls;
            let mode = match (sequential, ephemeral, container) {
                (true, true, _) => CreateMode::EphemeralSequential,
                (true, false, _) => CreateMode::PersistentSequential,
                (false, true, _) => CreateMode::Ephemeral,
                (false, false, true) => CreateMode::Container,
                _ => CreateMode::Persistent,
            };
            let options = mode.with_acls(Acls::anyone_all());
            let (_stat, sequence) = client.create(path, data.as_bytes(), &options).await?;
            let actual = if *sequential {
                format!("{path}{sequence}")
            } else {
                path.clone()
            };
            Ok(format!("Created {actual}"))
        }
        Command::Delete { path, version } => {
            client.delete(path, *version).await?;
            Ok("OK".to_string())
        }
        Command::DeleteAll { path } => {
            let outcome = manager.delete_node(connection_id, path, true).await?;
            if let Some(error) = outcome.failed_error {
                return Err(Box::new(error));
            }
            Ok(format!("Deleted {} nodes", outcome.deleted))
        }
        Command::Stat { path } => {
            let (_, node_stat) = client.get_children(path).await?;
            Ok(format_stat(path, &node_stat))
        }
        Command::GetAcl { path, stat } => {
            let (acls, node_stat) = client.get_acl(path).await?;
            let mut output = acls.iter().map(format_acl).collect::<Vec<_>>().join("\n");
            if *stat {
                output.push_str(&format_stat(path, &node_stat));
            }
            Ok(output)
        }
        Command::SetAcl {
            path,
            acl,
            version,
            recursive,
        } => {
            let acls = parse_acl(acl)?;
            if *recursive {
                let mut queue = std::collections::VecDeque::from([path.clone()]);
                while let Some(current) = queue.pop_front() {
                    client.set_acl(&current, &acls, *version).await?;
                    let (children, _) = client.get_children(&current).await?;
                    for child in children {
                        queue.push_back(if current == "/" {
                            format!("/{child}")
                        } else {
                            format!("{current}/{child}")
                        });
                    }
                }
            } else {
                client.set_acl(path, &acls, *version).await?;
            }
            Ok("OK".to_string())
        }
        Command::AllChildrenNumber { path } => {
            let mut queue = std::collections::VecDeque::from([path.clone()]);
            let mut count = 0u64;
            while let Some(current) = queue.pop_front() {
                let (children, _) = client.get_children(&current).await?;
                count += children.len() as u64;
                for child in children {
                    queue.push_back(if current == "/" {
                        format!("/{child}")
                    } else {
                        format!("{current}/{child}")
                    });
                }
            }
            Ok(count.to_string())
        }
        Command::Ephemerals { path } => {
            let paths = client.list_ephemerals(path).await?;
            Ok(paths.join("\n"))
        }
        Command::ListQuota { path } => {
            let quota = if path == "/" {
                "/zookeeper/quota".to_string()
            } else {
                format!("/zookeeper/quota{path}")
            };
            let limits = format!("{quota}/zookeeper_limits");
            match client.get_data(&limits).await {
                Ok((data, _)) => Ok(String::from_utf8_lossy(&data).to_string()),
                Err(zookeeper_client::Error::NoNode) => Ok("No quota set".to_string()),
                Err(error) => Err(Box::new(error.into())),
            }
        }
    }
}

fn help_text() -> String {
    [
        "ls [-R] [-s] [path]",
        "get [-s] path",
        "set [-v version] [-s] path data",
        "create [-s] [-e] [-c] path data",
        "delete [-v version] path",
        "deleteall path",
        "stat path",
        "getAcl [-s] path",
        "setAcl [-R] [-v version] path scheme:id:perms",
        "addauth scheme auth",
        "sync path",
        "getAllChildrenNumber path",
        "getEphemerals path",
        "listquota path",
        "version",
        "whoami",
        "help",
    ]
    .join("\n")
}

fn format_acl(acl: &Acl) -> String {
    format!(
        "{}.{}:{}",
        acl.scheme(),
        acl.id(),
        format_permissions(acl.permission())
    )
}

fn format_permissions(permission: Permission) -> String {
    let mut value = String::new();
    for (letter, flag) in [
        ("c", Permission::CREATE),
        ("r", Permission::READ),
        ("w", Permission::WRITE),
        ("d", Permission::DELETE),
        ("a", Permission::ADMIN),
    ] {
        if permission.has(flag) {
            value.push_str(letter);
        }
    }
    value
}

fn parse_acl(value: &str) -> AppResult<Vec<Acl>> {
    let mut acls = Vec::new();
    for entry in value.split(',') {
        let parts: Vec<&str> = entry.split(':').collect();
        if parts.len() != 3 {
            return Err(invalid("ACL must be scheme:id:perms"));
        }
        let mut combined: Option<Permission> = None;
        for letter in parts[2].chars() {
            let flag = match letter {
                'c' => Permission::CREATE,
                'r' => Permission::READ,
                'w' => Permission::WRITE,
                'd' => Permission::DELETE,
                'a' => Permission::ADMIN,
                _ => return Err(invalid("ACL permissions must use c/r/w/d/a")),
            };
            combined = Some(match combined {
                Some(current) => current | flag,
                None => flag,
            });
        }
        let permission = combined.ok_or_else(|| invalid("ACL permissions must not be empty"))?;
        acls.push(Acl::new(permission, AuthId::new(parts[0], parts[1])));
    }
    Ok(acls)
}

fn format_stat(path: &str, stat: &Stat) -> String {
    format!(
        "\ncZxid = {}\nctime = {}\nmZxid = {}\nmtime = {}\npZxid = {}\ncversion = {}\nversion = {}\naversion = {}\nephemeralOwner = {}\ndataLength = {}\nnumChildren = {}\npath = {path}\n",
        stat.czxid,
        stat.ctime,
        stat.mzxid,
        stat.mtime,
        stat.pzxid,
        stat.cversion,
        stat.version,
        stat.aversion,
        stat.ephemeral_owner,
        stat.data_length,
        stat.num_children
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ls_flags() {
        assert_eq!(
            parse("ls -R -s /app").unwrap(),
            Command::Ls {
                path: "/app".to_string(),
                recursive: true,
                stat: true
            }
        );
    }

    #[test]
    fn rejects_watcher_flags() {
        assert!(parse("get -w /app").is_err());
        assert!(parse("ls -w /").is_err());
    }

    #[test]
    fn parses_quoted_data() {
        assert_eq!(
            parse("create /app \"hello world\"").unwrap(),
            Command::Create {
                path: "/app".to_string(),
                data: "hello world".to_string(),
                sequential: false,
                ephemeral: false,
                container: false
            }
        );
    }

    #[test]
    fn rejects_unsupported_command() {
        assert!(parse("reconfig").is_err());
    }
}
