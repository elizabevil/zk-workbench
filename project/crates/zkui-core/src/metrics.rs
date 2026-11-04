//! 进程内指标：连接与节点操作的计数和延迟分位。

use std::sync::atomic::{AtomicU64, Ordering};

use crate::ipc::metrics::MetricsSnapshot;

const BUCKETS_MS: [u64; 7] = [1, 5, 10, 50, 100, 500, 1000];

/// 轻量计数器，仅本地使用。
#[derive(Default)]
pub struct Metrics {
    connect_attempts: AtomicU64,
    connect_failures: AtomicU64,
    reconnects: AtomicU64,
    node_reads: AtomicU64,
    node_writes: AtomicU64,
    node_creates: AtomicU64,
    node_deletes: AtomicU64,
    read_buckets: [AtomicU64; 8],
    write_buckets: [AtomicU64; 8],
}

impl Metrics {
    pub fn record_connect_attempt(&self) {
        self.connect_attempts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_connect_failure(&self) {
        self.connect_failures.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_reconnect(&self) {
        self.reconnects.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_read(&self, elapsed_ms: u64) {
        self.node_reads.fetch_add(1, Ordering::Relaxed);
        self.read_buckets[bucket_index(elapsed_ms)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_write(&self, elapsed_ms: u64) {
        self.node_writes.fetch_add(1, Ordering::Relaxed);
        self.write_buckets[bucket_index(elapsed_ms)].fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_create(&self) {
        self.node_creates.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_delete(&self, count: u64) {
        self.node_deletes.fetch_add(count, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            connect_attempts: self.connect_attempts.load(Ordering::Relaxed),
            connect_failures: self.connect_failures.load(Ordering::Relaxed),
            reconnects: self.reconnects.load(Ordering::Relaxed),
            node_reads: self.node_reads.load(Ordering::Relaxed),
            node_writes: self.node_writes.load(Ordering::Relaxed),
            node_creates: self.node_creates.load(Ordering::Relaxed),
            node_deletes: self.node_deletes.load(Ordering::Relaxed),
            read_p50_ms: percentile(&self.read_buckets, 50),
            read_p95_ms: percentile(&self.read_buckets, 95),
            write_p50_ms: percentile(&self.write_buckets, 50),
            write_p95_ms: percentile(&self.write_buckets, 95),
        }
    }
}

fn bucket_index(elapsed_ms: u64) -> usize {
    BUCKETS_MS
        .iter()
        .position(|upper| elapsed_ms <= *upper)
        .unwrap_or(BUCKETS_MS.len())
}

fn percentile(buckets: &[AtomicU64; 8], percentile: u64) -> u64 {
    let total: u64 = buckets
        .iter()
        .map(|count| count.load(Ordering::Relaxed))
        .sum();
    if total == 0 {
        return 0;
    }
    let target = total.saturating_mul(percentile).div_ceil(100);
    let mut seen = 0u64;
    for (index, count) in buckets.iter().enumerate() {
        seen += count.load(Ordering::Relaxed);
        if seen >= target {
            return BUCKETS_MS.get(index).copied().unwrap_or(1000);
        }
    }
    1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentiles_from_buckets() {
        let metrics = Metrics::default();
        metrics.record_read(2);
        metrics.record_read(8);
        metrics.record_read(80);
        let snapshot = metrics.snapshot();
        assert_eq!(snapshot.node_reads, 3);
        assert_eq!(snapshot.read_p50_ms, 10);
        assert_eq!(snapshot.read_p95_ms, 100);
    }

    #[test]
    fn empty_snapshot_is_zero() {
        let snapshot = Metrics::default().snapshot();
        assert_eq!(snapshot.read_p50_ms, 0);
        assert_eq!(snapshot.node_reads, 0);
    }
}
