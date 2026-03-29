// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for core error types (`NestGateUnifiedError` and detail structures).

use super::core_errors::*;
use std::time::Duration;

macro_rules! assert_display_contains {
    ($err:expr, $needle:expr) => {
        let d = $err.to_string();
        assert!(d.contains($needle), "expected {} in {}", $needle, d);
    };
}

#[test]
fn display_nest_gate_unified_error_configuration() {
    assert_display_contains!(
        NestGateUnifiedError::configuration_error("f", "m"),
        "Configuration error:"
    );
}

#[test]
fn display_nest_gate_unified_error_api() {
    assert_display_contains!(NestGateUnifiedError::api_error("x"), "API error:");
}

#[test]
fn display_nest_gate_unified_error_storage() {
    assert_display_contains!(NestGateUnifiedError::storage_error("x"), "Storage error:");
}

#[test]
fn display_nest_gate_unified_error_network() {
    assert_display_contains!(NestGateUnifiedError::network_error("x"), "Network error:");
}

#[test]
fn display_nest_gate_unified_error_security() {
    assert_display_contains!(NestGateUnifiedError::security_error("x"), "Security error:");
}

#[test]
fn display_nest_gate_unified_error_automation() {
    assert_display_contains!(
        NestGateUnifiedError::Automation(Box::new(AutomationErrorDetails {
            message: "a".into(),
            operation: None,
            target: None,
            automation_data: None,
            context: None,
        })),
        "Automation error:"
    );
}

#[test]
fn display_nest_gate_unified_error_system() {
    assert_display_contains!(
        NestGateUnifiedError::System(Box::new(SystemErrorDetails {
            message: "s".into(),
            component: "c".into(),
            operation: None,
            context: None,
        })),
        "System error:"
    );
}

#[test]
fn display_nest_gate_unified_error_internal() {
    assert_display_contains!(
        NestGateUnifiedError::Internal(Box::new(InternalErrorDetails {
            message: "i".into(),
            component: "c".into(),
            location: None,
            is_bug: false,
            context: None,
        })),
        "Internal error:"
    );
}

#[test]
fn display_nest_gate_unified_error_external() {
    assert_display_contains!(
        NestGateUnifiedError::External(Box::new(ExternalErrorDetails {
            message: "e".into(),
            service: "svc".into(),
            retryable: true,
            context: None,
        })),
        "External error:"
    );
}

#[test]
fn display_nest_gate_unified_error_validation() {
    assert_display_contains!(
        NestGateUnifiedError::validation_error("v"),
        "Validation error:"
    );
}

#[test]
fn display_nest_gate_unified_error_timeout() {
    assert_display_contains!(
        NestGateUnifiedError::timeout_error("op", Duration::from_secs(1)),
        "Timeout error:"
    );
}

#[test]
fn display_nest_gate_unified_error_io() {
    assert_display_contains!(
        NestGateUnifiedError::Io(Box::new(IoErrorDetails {
            message: "io".into(),
            path: None,
            operation: None,
            context: None,
        })),
        "I/O error:"
    );
}

#[test]
fn display_nest_gate_unified_error_resource_exhausted() {
    assert_display_contains!(
        NestGateUnifiedError::ResourceExhausted(Box::new(ResourceExhaustedErrorDetails {
            message: "r".into(),
            resource: "cpu".into(),
            current: None,
            limit: None,
            context: None,
        })),
        "Resource exhausted:"
    );
}

#[test]
fn display_nest_gate_unified_error_testing() {
    assert_display_contains!(
        NestGateUnifiedError::Testing(Box::new(TestingErrorDetails {
            message: "t".into(),
            test_name: None,
            test_type: Some(TestType::Unit),
            assertion_failure: None,
            expected: None,
            actual: None,
            test_data: None,
            context: None,
        })),
        "Testing error:"
    );
}

#[test]
fn display_nest_gate_unified_error_performance() {
    assert_display_contains!(
        NestGateUnifiedError::Performance(Box::new(PerformanceErrorDetails {
            message: "p".into(),
            operation: "op".into(),
            metric: None,
            expected: None,
            actual: None,
            unit: None,
            performance_data: None,
            context: None,
        })),
        "Performance error:"
    );
}

#[test]
fn display_nest_gate_unified_error_handler() {
    assert_display_contains!(
        NestGateUnifiedError::Handler(Box::new(HandlerErrorDetails {
            message: "h".into(),
            handler_name: "hn".into(),
            request_info: None,
            handler_data: None,
            context: None,
        })),
        "Handler error"
    );
}

#[test]
fn display_nest_gate_unified_error_load_balancer() {
    assert_display_contains!(
        NestGateUnifiedError::LoadBalancer(Box::new(LoadBalancerErrorDetails {
            message: "lb".into(),
            available_services: None,
            algorithm: None,
        })),
        "Load balancer error:"
    );
}

#[test]
fn display_nest_gate_unified_error_not_implemented() {
    assert_display_contains!(
        NestGateUnifiedError::NotImplemented(Box::new(NotImplementedErrorDetails {
            feature: "f".into(),
            message: None,
            planned_version: None,
        })),
        "Not implemented:"
    );
}

#[test]
fn constructor_helpers_non_empty_display() {
    let samples = [
        NestGateUnifiedError::network_connection_failed("h", 80, "refused").to_string(),
        NestGateUnifiedError::network_timeout("http://x", Duration::from_secs(2)).to_string(),
        NestGateUnifiedError::storage_not_found("/p").to_string(),
        NestGateUnifiedError::storage_permission_denied("/p", "read").to_string(),
        NestGateUnifiedError::storage_disk_full("/p", 100, 10).to_string(),
        NestGateUnifiedError::validation_field("f", "bad").to_string(),
        NestGateUnifiedError::validation_schema("s", "bad", None).to_string(),
        NestGateUnifiedError::security_authentication_failed("u", "bad").to_string(),
        NestGateUnifiedError::security_authorization_failed("u", "act", "res").to_string(),
        NestGateUnifiedError::security_encryption_failed("aes", "bad").to_string(),
        NestGateUnifiedError::api_not_found("/x").to_string(),
        NestGateUnifiedError::api_bad_request("bad").to_string(),
        NestGateUnifiedError::api_internal_error("bad").to_string(),
        NestGateUnifiedError::configuration_invalid_value("f", "v", "e").to_string(),
        NestGateUnifiedError::configuration_missing_required("f").to_string(),
        NestGateUnifiedError::feature_not_enabled("f", "msg").to_string(),
        NestGateUnifiedError::storage_operation("op", true).to_string(),
        NestGateUnifiedError::simple("quick").to_string(),
    ];
    for s in samples {
        assert!(!s.is_empty());
    }
}

#[test]
fn detailed_constructors_roundtrip_serde() {
    let e = NestGateUnifiedError::configuration_error_detailed(
        "field",
        "msg",
        Some("cur".into()),
        Some("exp".into()),
        true,
    );
    let json = serde_json::to_string(&e).unwrap();
    let back: NestGateUnifiedError = serde_json::from_str(&json).unwrap();
    assert_eq!(e.to_string(), back.to_string());
}

#[test]
fn error_severity_all_variants_debug_and_clone() {
    for s in [
        ErrorSeverity::Low,
        ErrorSeverity::Medium,
        ErrorSeverity::High,
        ErrorSeverity::Critical,
    ] {
        let c = s.clone();
        assert_eq!(format!("{s:?}"), format!("{c:?}"));
    }
}

#[test]
fn configuration_error_details_display_and_clone() {
    let d = ConfigurationErrorDetails {
        field: "f".into(),
        message: "m".into(),
        currentvalue: Some("c".into()),
        expected: Some("e".into()),
        user_error: true,
    };
    let _ = d.to_string();
    let d2 = d.clone();
    assert_eq!(d.field, d2.field);
}

macro_rules! r6_assert_nonempty {
    ($($e:expr),+ $(,)?) => {
        $(assert!(!$e.to_string().is_empty(), "empty display");)+
    };
}

#[test]
fn r6_display_automation_system_external() {
    r6_assert_nonempty!(
        NestGateUnifiedError::Automation(Box::new(AutomationErrorDetails {
            message: "a".into(),
            operation: Some("op".into()),
            target: Some("t".into()),
            automation_data: None,
            context: None,
        })),
        NestGateUnifiedError::System(Box::new(SystemErrorDetails {
            message: "s".into(),
            component: "c".into(),
            operation: Some("o".into()),
            context: None,
        })),
        NestGateUnifiedError::External(Box::new(ExternalErrorDetails {
            message: "e".into(),
            service: "svc".into(),
            retryable: false,
            context: None,
        }))
    );
}

#[test]
fn r6_timeout_io_details_display() {
    let t = TimeoutErrorDetails {
        message: "t".into(),
        operation: Some("o".into()),
        timeout: Duration::from_nanos(1),
        retryable: false,
        context: None,
    };
    assert!(!t.to_string().is_empty());
    let io = IoErrorDetails {
        message: "i".into(),
        path: Some("/p".into()),
        operation: Some("read".into()),
        context: None,
    };
    assert!(!io.to_string().is_empty());
}

#[test]
fn r6_resource_testing_details_display() {
    let r = ResourceExhaustedErrorDetails {
        message: "m".into(),
        resource: "r".into(),
        current: Some(1),
        limit: Some(2),
        context: None,
    };
    assert!(!r.to_string().is_empty());
    let te = TestingErrorDetails {
        message: "m".into(),
        test_name: Some("n".into()),
        test_type: Some(TestType::Integration),
        assertion_failure: Some("a".into()),
        expected: Some("e".into()),
        actual: Some("a".into()),
        test_data: None,
        context: None,
    };
    assert!(!te.to_string().is_empty());
}

#[test]
fn r6_performance_handler_lb_not_impl_display() {
    let p = PerformanceErrorDetails {
        message: "m".into(),
        operation: "op".into(),
        metric: Some("met".into()),
        expected: Some(1.0),
        actual: Some(2.0),
        unit: Some("ms".into()),
        performance_data: None,
        context: None,
    };
    assert!(!p.to_string().is_empty());
    let h = HandlerErrorDetails {
        message: "m".into(),
        handler_name: "h".into(),
        request_info: Some("r".into()),
        handler_data: None,
        context: None,
    };
    assert!(!h.to_string().is_empty());
    let l = LoadBalancerErrorDetails {
        message: "m".into(),
        available_services: Some(0),
        algorithm: Some("rr".into()),
    };
    assert!(!l.to_string().is_empty());
    let n = NotImplementedErrorDetails {
        feature: "f".into(),
        message: Some("msg".into()),
        planned_version: Some("v".into()),
    };
    assert!(!n.to_string().is_empty());
}

#[test]
fn r6_api_storage_network_validation_details_serde() {
    let a = ApiErrorDetails {
        message: "m".into(),
        status_code: Some(418),
        request_id: Some("id".into()),
        endpoint: Some("/e".into()),
        context: None,
    };
    let json = serde_json::to_string(&a).unwrap();
    let _: ApiErrorDetails = serde_json::from_str(&json).unwrap();

    let s = StorageErrorDetails {
        message: "m".into(),
        operation: Some("o".into()),
        resource: Some("r".into()),
        storage_data: None,
        context: None,
    };
    let _ = serde_json::to_string(&s).unwrap();

    let v = ValidationErrorDetails {
        message: "m".into(),
        field: Some("f".into()),
        expected: Some("e".into()),
        actual: Some("a".into()),
        context: None,
    };
    let _ = serde_json::to_string(&v).unwrap();
}

#[test]
fn r6_security_automation_network_details_roundtrip() {
    let sec = SecurityErrorDetails {
        message: "m".into(),
        operation: Some("o".into()),
        principal: Some("p".into()),
        security_data: None,
        context: None,
    };
    let _ = serde_json::to_string(&sec).unwrap();
    let net = NetworkErrorDetails {
        message: "m".into(),
        operation: Some("o".into()),
        endpoint: Some("e".into()),
        network_data: None,
        context: None,
    };
    let _ = serde_json::to_string(&net).unwrap();
}

#[test]
fn r6_internal_error_details_fields() {
    let i = InternalErrorDetails {
        message: "m".into(),
        component: "c".into(),
        location: Some("loc".into()),
        is_bug: true,
        context: None,
    };
    assert!(i.to_string().contains("m"));
}

#[test]
fn r6_test_type_all_variants_serde() {
    for t in [
        TestType::Unit,
        TestType::Integration,
        TestType::E2E,
        TestType::Performance,
        TestType::Security,
        TestType::Chaos,
    ] {
        let j = serde_json::to_string(&t).unwrap();
        let _: TestType = serde_json::from_str(&j).unwrap();
    }
}

#[test]
fn r6_nest_gate_unified_error_serde_roundtrip_variants() {
    let samples = vec![
        NestGateUnifiedError::configuration_error("f", "m"),
        NestGateUnifiedError::api_error("a"),
        NestGateUnifiedError::simple("s"),
    ];
    for e in samples {
        let j = serde_json::to_string(&e).unwrap();
        let back: NestGateUnifiedError = serde_json::from_str(&j).unwrap();
        assert_eq!(e.to_string(), back.to_string());
    }
}

macro_rules! r6_smoke {
    ($($name:ident => $e:expr);+ $(;)?) => {
        $(
            #[test]
            fn $name() {
                assert!(!$e.to_string().is_empty());
            }
        )+
    };
}

r6_smoke! {
    r6_smoke_cfg01 => NestGateUnifiedError::configuration_error("a", "b");
    r6_smoke_cfg02 => NestGateUnifiedError::configuration_missing_required("req");
    r6_smoke_api01 => NestGateUnifiedError::api_bad_request("br");
    r6_smoke_api02 => NestGateUnifiedError::api_internal_error("ie");
    r6_smoke_api03 => NestGateUnifiedError::api_not_found("/x");
    r6_smoke_net01 => NestGateUnifiedError::network_error("n");
    r6_smoke_net02 => NestGateUnifiedError::network_connection_failed("h", 443, "x");
    r6_smoke_net03 => NestGateUnifiedError::network_timeout("http://u", Duration::from_secs(1));
    r6_smoke_sec01 => NestGateUnifiedError::security_error("s");
    r6_smoke_sec02 => NestGateUnifiedError::security_authentication_failed("u", "bad");
    r6_smoke_sec03 => NestGateUnifiedError::security_authorization_failed("u", "a", "r");
    r6_smoke_sec04 => NestGateUnifiedError::security_encryption_failed("aes", "e");
    r6_smoke_sto01 => NestGateUnifiedError::storage_error("st");
    r6_smoke_sto02 => NestGateUnifiedError::storage_not_found("/p");
    r6_smoke_sto03 => NestGateUnifiedError::storage_permission_denied("/p", "w");
    r6_smoke_sto04 => NestGateUnifiedError::storage_disk_full("/p", 100, 1);
    r6_smoke_sto05 => NestGateUnifiedError::storage_operation("op", false);
    r6_smoke_val01 => NestGateUnifiedError::validation_error("v");
    r6_smoke_val02 => NestGateUnifiedError::validation_field("f", "bad");
    r6_smoke_val03 => NestGateUnifiedError::validation_schema("s", "bad", Some("p".into()));
    r6_smoke_to01 => NestGateUnifiedError::timeout_error("op", Duration::from_millis(500));
    r6_smoke_feat01 => NestGateUnifiedError::feature_not_enabled("feat", "msg");
    r6_smoke_cfg_inv => NestGateUnifiedError::configuration_invalid_value("f", "v", "e");
    r6_smoke_sim01 => NestGateUnifiedError::simple("quick");
    r6_smoke_sim02 => NestGateUnifiedError::simple("another");
    r6_smoke_det_api => NestGateUnifiedError::api_error_detailed(
        "m",
        Some(502),
        Some("rid".into()),
        Some("/e".into()),
    );
    r6_smoke_det_cfg => NestGateUnifiedError::configuration_error_detailed(
        "f",
        "msg",
        Some("c".into()),
        Some("e".into()),
        false,
    );
    r6_smoke_det_sto => NestGateUnifiedError::storage_error_detailed("m", Some("o".into()));
    r6_smoke_det_net => NestGateUnifiedError::network_error_detailed(
        "m",
        Some("o".into()),
        Some("ep".into()),
    );
    r6_smoke_det_val => NestGateUnifiedError::validation_error_detailed(
        "m",
        Some("f".into()),
        Some("e".into()),
        Some("a".into()),
    );
}

#[test]
fn error_severity_roundtrip_serde() {
    for sev in [
        ErrorSeverity::Low,
        ErrorSeverity::Medium,
        ErrorSeverity::High,
        ErrorSeverity::Critical,
    ] {
        let s = serde_json::to_string(&sev).expect("serialize");
        let back: ErrorSeverity = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back, sev);
    }
}

#[test]
fn configuration_error_constructor_and_display() {
    let e = NestGateUnifiedError::configuration_error("port", "invalid");
    assert!(e.to_string().contains("Configuration"));
    assert!(e.to_string().contains("port"));
}

#[test]
fn api_storage_network_validation_constructors_roundtrip_serde() {
    let cases = vec![
        NestGateUnifiedError::api_error("bad"),
        NestGateUnifiedError::storage_error("io fail"),
        NestGateUnifiedError::network_error("timeout"),
        NestGateUnifiedError::security_error("denied"),
        NestGateUnifiedError::validation_error("schema"),
    ];
    for err in cases {
        let s = serde_json::to_string(&err).expect("serialize");
        let back: NestGateUnifiedError = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(format!("{err}"), format!("{back}"));
    }
}

#[test]
fn timeout_error_includes_duration() {
    let e = NestGateUnifiedError::timeout_error("job", Duration::from_millis(0));
    let s = e.to_string();
    assert!(s.contains("job") || s.contains("timed out"));
}

#[test]
fn detailed_constructors_preserve_fields() {
    let cfg = NestGateUnifiedError::configuration_error_detailed(
        "k".to_string(),
        "m".to_string(),
        Some("cur".into()),
        Some("exp".into()),
        true,
    );
    let json = serde_json::to_string(&cfg).unwrap();
    assert!(json.contains("k"));

    let api = NestGateUnifiedError::api_error_detailed(
        "msg".to_string(),
        Some(418),
        Some("rid".into()),
        Some("/x".into()),
    );
    let api_s = api.to_string();
    assert!(api_s.contains("418") || api_s.contains("msg"));

    let v = NestGateUnifiedError::validation_error_detailed(
        "v".to_string(),
        Some("f".into()),
        Some("e".into()),
        Some("a".into()),
    );
    assert!(v.to_string().contains("Validation"));
}

#[test]
fn migration_helpers_messages() {
    let nc = NestGateUnifiedError::network_connection_failed("10.0.0.1", 443, "refused");
    assert!(nc.to_string().contains("10.0.0.1") || nc.to_string().contains("443"));

    let nt = NestGateUnifiedError::network_timeout("https://x", Duration::from_secs(1));
    assert!(nt.to_string().contains("timeout") || nt.to_string().contains("https://x"));

    let sn = NestGateUnifiedError::storage_not_found("/missing");
    assert!(sn.to_string().contains("/missing"));

    let sp = NestGateUnifiedError::storage_permission_denied("/data", "write");
    assert!(sp.to_string().contains("Permission"));

    let df = NestGateUnifiedError::storage_disk_full("/z", 100, 1);
    assert!(df.to_string().contains("Disk") || df.to_string().contains("disk"));

    let vf = NestGateUnifiedError::validation_field("email", "bad");
    assert!(vf.to_string().contains("email"));

    let vs = NestGateUnifiedError::validation_schema("v1", "oops", Some("/a".into()));
    assert!(vs.to_string().contains("Schema") || vs.to_string().contains("v1"));

    let sa = NestGateUnifiedError::security_authentication_failed("u", "nope");
    assert!(sa.to_string().contains("Authentication"));

    let sz = NestGateUnifiedError::security_authorization_failed("u", "delete", "ds");
    assert!(sz.to_string().contains("Authorization"));

    let se = NestGateUnifiedError::security_encryption_failed("aes", "bad key");
    assert!(se.to_string().contains("Encryption"));

    let nf = NestGateUnifiedError::api_not_found("/api/x");
    assert!(nf.to_string().contains("404") || nf.to_string().contains("not found"));

    let br = NestGateUnifiedError::api_bad_request("x");
    assert!(br.to_string().contains("400") || br.to_string().contains("Bad"));

    let ie = NestGateUnifiedError::api_internal_error("boom");
    assert!(ie.to_string().contains("500") || ie.to_string().contains("Internal"));

    let iv = NestGateUnifiedError::configuration_invalid_value("f", "v", "e");
    assert!(iv.to_string().contains("Invalid"));

    let mr = NestGateUnifiedError::configuration_missing_required("req_field");
    assert!(mr.to_string().contains("req_field"));

    let fe = NestGateUnifiedError::feature_not_enabled("feat", "off");
    assert!(fe.to_string().contains("feat") || fe.to_string().contains("Not implemented"));

    let so = NestGateUnifiedError::storage_operation("op", false);
    assert!(so.to_string().contains("Storage") || so.to_string().contains("op"));

    let sim = NestGateUnifiedError::simple("hello");
    assert!(sim.to_string().contains("hello") || sim.to_string().contains("Internal"));
}

#[test]
fn test_type_serde() {
    let t = TestType::Performance;
    let s = serde_json::to_string(&t).unwrap();
    let back: TestType = serde_json::from_str(&s).unwrap();
    let s2 = serde_json::to_string(&back).unwrap();
    assert_eq!(s, s2);
}

#[test]
fn storage_error_detailed_network_detailed_roundtrip() {
    let s = NestGateUnifiedError::storage_error_detailed("m".to_string(), Some("snap".into()));
    let n =
        NestGateUnifiedError::network_error_detailed("m2", Some("op".into()), Some("ep".into()));
    for e in [s, n] {
        let j = serde_json::to_string(&e).unwrap();
        let _: NestGateUnifiedError = serde_json::from_str(&j).unwrap();
    }
}

#[test]
fn automation_security_storage_extension_constructors_display() {
    assert!(
        NestGateUnifiedError::automation("job")
            .to_string()
            .contains("Automation")
    );
    assert!(
        NestGateUnifiedError::automation_operation("job", Some("tgt".into()))
            .to_string()
            .contains("Automation")
    );
    assert!(
        NestGateUnifiedError::auth("no")
            .to_string()
            .contains("Security")
    );
    assert!(
        NestGateUnifiedError::security("s")
            .to_string()
            .contains("Security")
    );
    assert!(
        NestGateUnifiedError::authorization("denied", "alice")
            .to_string()
            .contains("Security")
    );
    assert!(
        NestGateUnifiedError::storage("blob")
            .to_string()
            .contains("Storage")
    );
    assert!(
        NestGateUnifiedError::storage_with_operation("x", "put")
            .to_string()
            .contains("Storage")
    );
}
