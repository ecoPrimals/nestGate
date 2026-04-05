// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

mod cli_parse_tests {
    use crate::cli::{Cli, Commands, ConfigAction, DiscoverTarget, ServiceAction, StorageAction};
    use clap::Parser;
    use std::net::SocketAddr;
    use std::path::PathBuf;

    #[test]
    fn parses_version_subcommand() {
        let cli = Cli::try_parse_from(["nestgate", "version"]).expect("parse");
        assert!(matches!(cli.command, Commands::Version));
    }

    #[test]
    fn parses_status_and_health() {
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "status"])
                .expect("parse status")
                .command,
            Commands::Status
        ));
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "health"])
                .expect("parse health")
                .command,
            Commands::Health
        ));
    }

    #[test]
    fn daemon_accepts_server_alias() {
        let cli = Cli::try_parse_from(["nestgate", "server", "--dev"]).expect("parse server alias");
        match cli.command {
            Commands::Daemon { dev, .. } => assert!(dev),
            _ => panic!("expected daemon via server alias"),
        }
    }

    #[test]
    fn parses_service_status() {
        let cli = Cli::try_parse_from(["nestgate", "service", "status"]).expect("parse");
        match cli.command {
            Commands::Service { action } => assert!(matches!(action, ServiceAction::Status)),
            _ => panic!("expected service"),
        }
    }

    #[test]
    fn parses_service_start_with_listen() {
        let cli = Cli::try_parse_from([
            "nestgate",
            "service",
            "start",
            "--port",
            "3000",
            "--bind",
            "127.0.0.1",
            "--listen",
            "127.0.0.1:4000",
        ])
        .expect("parse");
        match cli.command {
            Commands::Service { action } => match action {
                ServiceAction::Start {
                    port, bind, listen, ..
                } => {
                    assert_eq!(port, 3000);
                    assert_eq!(bind, "127.0.0.1");
                    assert_eq!(
                        listen,
                        Some("127.0.0.1:4000".parse::<SocketAddr>().expect("socket addr"))
                    );
                }
                _ => panic!("start"),
            },
            _ => panic!("service"),
        }
    }

    #[test]
    fn global_verbose_and_config() {
        let cli = Cli::try_parse_from([
            "nestgate",
            "-v",
            "--config",
            "/tmp/ng.toml",
            "--output",
            "json",
            "health",
        ])
        .expect("parse");
        assert!(cli.verbose);
        assert_eq!(cli.config, Some(PathBuf::from("/tmp/ng.toml")));
        assert_eq!(cli.output, "json");
        assert!(matches!(cli.command, Commands::Health));
    }

    #[test]
    fn daemon_parses_socket_only_and_family() {
        let cli = Cli::try_parse_from([
            "nestgate",
            "daemon",
            "--family-id",
            "fam-a",
            "--enable-http",
        ])
        .expect("parse");
        match cli.command {
            Commands::Daemon {
                family_id,
                enable_http,
                port,
                ..
            } => {
                assert_eq!(family_id.as_deref(), Some("fam-a"));
                assert!(enable_http);
                assert!(port.is_none());
            }
            _ => panic!("daemon"),
        }
    }

    #[test]
    fn daemon_parses_explicit_port_for_tcp_jsonrpc() {
        let cli = Cli::try_parse_from(["nestgate", "daemon", "--port", "9443"]).expect("parse");
        match cli.command {
            Commands::Daemon { port, .. } => assert_eq!(port, Some(9443)),
            _ => panic!("daemon"),
        }
    }

    #[test]
    fn service_logs_defaults() {
        let cli = Cli::try_parse_from(["nestgate", "service", "logs"]).expect("parse");
        match cli.command {
            Commands::Service { action } => match action {
                ServiceAction::Logs { lines, follow } => {
                    assert_eq!(lines, 100);
                    assert!(!follow);
                }
                _ => panic!("logs"),
            },
            _ => panic!("service"),
        }
    }

    #[test]
    fn parses_doctor_storage_config_discover() {
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "doctor", "--comprehensive"])
                .expect("parse doctor")
                .command,
            Commands::Doctor {
                comprehensive: true,
                fix: false
            }
        ));
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "storage", "list"])
                .expect("parse storage list")
                .command,
            Commands::Storage { .. }
        ));
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "config", "validate"])
                .expect("parse config validate")
                .command,
            Commands::Config { .. }
        ));
        assert!(matches!(
            Cli::try_parse_from(["nestgate", "discover", "capabilities"])
                .expect("parse discover capabilities")
                .command,
            Commands::Discover { .. }
        ));
    }

    #[test]
    fn parses_zfs_list_datasets() {
        let cli = Cli::try_parse_from(["nestgate", "zfs", "list-datasets"]).expect("parse");
        assert!(matches!(cli.command, Commands::Zfs { .. }));
    }

    #[test]
    fn parses_storage_scan_with_flags() {
        let cli = Cli::try_parse_from([
            "nestgate",
            "storage",
            "scan",
            "--path",
            "/mnt",
            "--cloud",
            "--network",
        ])
        .expect("parse storage scan");
        match cli.command {
            Commands::Storage { action } => match action {
                StorageAction::Scan {
                    path,
                    cloud,
                    network,
                } => {
                    assert_eq!(path, PathBuf::from("/mnt"));
                    assert!(cloud);
                    assert!(network);
                }
                _ => panic!("expected scan"),
            },
            _ => panic!("expected storage"),
        }
    }

    #[test]
    fn parses_storage_benchmark() {
        let cli = Cli::try_parse_from([
            "nestgate",
            "storage",
            "benchmark",
            "zfs",
            "--duration",
            "10",
            "--size",
            "50",
        ])
        .expect("parse storage benchmark");
        match cli.command {
            Commands::Storage { action } => match action {
                StorageAction::Benchmark {
                    backend,
                    duration,
                    size,
                } => {
                    assert_eq!(backend, "zfs");
                    assert_eq!(duration, 10);
                    assert_eq!(size, 50);
                }
                _ => panic!("expected benchmark"),
            },
            _ => panic!("expected storage"),
        }
    }

    #[test]
    fn parses_config_set_and_get() {
        let set_cli = Cli::try_parse_from(["nestgate", "config", "set", "key_a", "value_b"])
            .expect("parse config set");
        match set_cli.command {
            Commands::Config { action } => match action {
                ConfigAction::Set { key, value } => {
                    assert_eq!(key, "key_a");
                    assert_eq!(value, "value_b");
                }
                _ => panic!("expected set"),
            },
            _ => panic!("expected config"),
        }

        let get_cli =
            Cli::try_parse_from(["nestgate", "config", "get", "key_a"]).expect("parse config get");
        match get_cli.command {
            Commands::Config { action } => match action {
                ConfigAction::Get { key } => assert_eq!(key, "key_a"),
                _ => panic!("expected get"),
            },
            _ => panic!("expected config"),
        }
    }

    #[test]
    fn parses_discover_primals_and_services() {
        let p = Cli::try_parse_from(["nestgate", "discover", "primals"]).expect("discover primals");
        match p.command {
            Commands::Discover { target } => {
                assert!(matches!(target, DiscoverTarget::Primals));
            }
            _ => panic!("discover"),
        }
        let s =
            Cli::try_parse_from(["nestgate", "discover", "services"]).expect("discover services");
        match s.command {
            Commands::Discover { target } => {
                assert!(matches!(target, DiscoverTarget::Services));
            }
            _ => panic!("discover"),
        }
    }

    #[test]
    fn parses_doctor_with_fix() {
        let cli = Cli::try_parse_from(["nestgate", "doctor", "--fix"]).expect("doctor fix");
        match cli.command {
            Commands::Doctor { comprehensive, fix } => {
                assert!(!comprehensive);
                assert!(fix);
            }
            _ => panic!("doctor"),
        }
    }

    #[test]
    fn parse_fails_on_unknown_subcommand() {
        let err = Cli::try_parse_from(["nestgate", "not-a-real-command"]);
        assert!(err.is_err(), "expected clap error for unknown subcommand");
    }

    #[test]
    fn parse_fails_on_daemon_conflicting_flags() {
        let err = Cli::try_parse_from(["nestgate", "daemon", "--socket-only", "--enable-http"]);
        assert!(err.is_err(), "socket_only conflicts with enable_http");
    }
}

mod port_env_tests {
    use crate::cli::{Cli, Commands};
    use crate::commands::env::{bind_from_env_source, port_from_env_source};
    use clap::Parser;
    use nestgate_core::constants::{DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS};
    use nestgate_types::MapEnv;
    use std::net::SocketAddr;

    #[test]
    fn port_from_env_prefers_api_over_http_and_port() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "7101"),
            ("NESTGATE_HTTP_PORT", "7102"),
            ("NESTGATE_PORT", "7103"),
        ]);
        assert_eq!(port_from_env_source(&env), 7101);
    }

    #[test]
    fn port_from_env_http_when_api_missing() {
        let env = MapEnv::from([("NESTGATE_HTTP_PORT", "7202"), ("NESTGATE_PORT", "7203")]);
        assert_eq!(port_from_env_source(&env), 7202);
    }

    #[test]
    fn port_from_env_nestgate_port_when_api_and_http_missing() {
        let env = MapEnv::from([("NESTGATE_PORT", "7303")]);
        assert_eq!(port_from_env_source(&env), 7303);
    }

    #[test]
    fn port_from_env_defaults_when_all_missing() {
        let env = MapEnv::new();
        assert_eq!(port_from_env_source(&env), DEFAULT_API_PORT);
    }

    #[test]
    fn port_from_env_invalid_api_string_uses_default_not_http() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "not-a-port"),
            ("NESTGATE_HTTP_PORT", "7402"),
        ]);
        assert_eq!(port_from_env_source(&env), DEFAULT_API_PORT);
    }

    #[test]
    fn port_from_env_out_of_range_uses_default() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "65536")]);
        assert_eq!(port_from_env_source(&env), DEFAULT_API_PORT);
    }

    #[test]
    fn bind_from_env_prefers_nestgate_bind() {
        let env = MapEnv::from([
            ("NESTGATE_BIND", "192.0.2.10"),
            ("NESTGATE_BIND_ADDRESS", "192.0.2.11"),
            ("NESTGATE_HOST", "192.0.2.12"),
        ]);
        assert_eq!(bind_from_env_source(&env), "192.0.2.10");
    }

    #[test]
    fn bind_from_env_falls_back_to_bind_address() {
        let env = MapEnv::from([
            ("NESTGATE_BIND_ADDRESS", "192.0.2.20"),
            ("NESTGATE_HOST", "192.0.2.21"),
        ]);
        assert_eq!(bind_from_env_source(&env), "192.0.2.20");
    }

    #[test]
    fn bind_from_env_falls_back_to_host() {
        let env = MapEnv::from([("NESTGATE_HOST", "192.0.2.30")]);
        assert_eq!(bind_from_env_source(&env), "192.0.2.30");
    }

    #[test]
    fn bind_from_env_defaults_when_all_missing() {
        let env = MapEnv::new();
        assert_eq!(bind_from_env_source(&env), DEFAULT_BIND_ADDRESS);
    }

    #[test]
    fn daemon_resolves_listen_socket_addr() {
        let cli = Cli::try_parse_from(["nestgate", "daemon", "--listen", "[::1]:8443"])
            .expect("parse daemon with listen");
        match cli.command {
            Commands::Daemon { listen, .. } => {
                assert_eq!(
                    listen,
                    Some("[::1]:8443".parse::<SocketAddr>().expect("listen addr"))
                );
            }
            _ => panic!("expected daemon"),
        }
    }
}

mod setup_and_banner_tests {
    use crate::cli::{print_banner, setup_logging};
    use serial_test::serial;

    #[test]
    #[serial]
    fn setup_logging_initializes_subscriber_once() {
        setup_logging(false);
    }

    #[test]
    fn print_banner_runs_without_panic() {
        print_banner();
    }
}
