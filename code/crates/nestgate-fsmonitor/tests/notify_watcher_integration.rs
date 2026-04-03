// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration tests: real `notify` watcher behavior (dependency of this crate).

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{RecvTimeoutError, channel};
use std::time::Duration;

#[test]
fn recommended_watcher_observes_new_file_in_temp_directory() {
    let dir = tempfile::tempdir().expect("create temp directory for watch root");
    let file_path = dir.path().join("notify_probe.txt");
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        Config::default(),
    )
    .expect("instantiate notify RecommendedWatcher");
    watcher
        .watch(dir.path(), RecursiveMode::NonRecursive)
        .expect("register watch on temp directory");

    std::fs::write(&file_path, b"nestgate-fsmonitor").expect("create file under watched dir");

    let deadline = Duration::from_secs(5);
    let started = std::time::Instant::now();
    let mut saw_target = false;
    while started.elapsed() < deadline {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(event)) => {
                let name = file_path.file_name().expect("file path has a file name");
                if event.paths.iter().any(|p| p.file_name() == Some(name)) {
                    saw_target = true;
                    break;
                }
            }
            Ok(Err(err)) => panic!("notify error: {err}"),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }

    drop(watcher);
    assert!(
        saw_target,
        "expected at least one notify event referencing {file_path:?}"
    );
}
