//! 连接状态机。

use serde::Serialize;
use ts_rs::TS;

/// 连接状态，至少覆盖创建中、连接中、已连接、断开或重连中、会话过期、关闭。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(
    rename_all = "snake_case",
    export,
    export_to = "../../../src/api/types/"
)]
pub enum ConnectionState {
    Creating,
    Connecting,
    Connected,
    Disconnected,
    Expired,
    Closed,
}

impl ConnectionState {
    /// 是否为终态。
    pub fn is_terminal(self) -> bool {
        matches!(self, ConnectionState::Expired | ConnectionState::Closed)
    }

    /// 是否允许迁移到下一状态。
    ///
    /// 规则保持宽松但显式：终态不可再迁移；其余状态可进入断开、过期与关闭。
    pub fn can_transition_to(self, next: ConnectionState) -> bool {
        if self == next {
            return true;
        }
        if self.is_terminal() {
            return false;
        }
        matches!(
            next,
            ConnectionState::Connecting
                | ConnectionState::Connected
                | ConnectionState::Disconnected
                | ConnectionState::Expired
                | ConnectionState::Closed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ConnectionState::*;

    #[test]
    fn terminal_states_cannot_transition() {
        assert!(!Closed.can_transition_to(Connected));
        assert!(!Expired.can_transition_to(Connected));
    }

    #[test]
    fn reconnect_path_is_allowed() {
        assert!(Connected.can_transition_to(Disconnected));
        assert!(Disconnected.can_transition_to(Connected));
        assert!(Disconnected.can_transition_to(Expired));
        assert!(Creating.can_transition_to(Connecting));
    }

    #[test]
    fn same_state_is_allowed() {
        assert!(Connected.can_transition_to(Connected));
    }
}
