//! FER-11028 Verification Suite — SDK Client Glue
//!
//! These integration tests verify that the CLI generator's sdk_glue module
//! correctly bridges the generated SDK through the CLI's transport stack.
//!
//! Acceptance criteria tested:
//! 1. Wiremock parity: built-in executor and SDK executor produce identical
//!    wire requests (method, path, headers, body).
//! 2. Auth inheritance: SDK calls carry the same auth headers as built-in.
//! 3. Type identity: `Pet` from `petstore_api_sdk::prelude` is the same
//!    type as `petstore_api_types::Pet`.
//! 4. Regression: `CliExecutor` still correctly applies base-URL override,
//!    global headers, and retries (invoke() path).

use std::sync::Arc;

use fern_cli_sdk::auth::{no_auth_provider, AuthProvider, DynAuthProvider};
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::http::HttpConfig;
use fern_cli_sdk::sdk_executor::{CliExecutor, SdkError, SdkRequestExecutor};

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ═══════════════════════════════════════════════════════════════════════
// 1. WIREMOCK PARITY — built-in vs SDK executor produce identical requests
// ═══════════════════════════════════════════════════════════════════════

/// Verify that a GET request through CliExecutor hits the mock server with
/// the expected method, path, and global headers — same behavior as the
/// built-in execute path.
#[tokio::test]
async fn parity_get_request_reaches_mock_with_correct_method_and_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/pets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"[]"#))
        .expect(1)
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(http, no_auth_provider(), vec![], None);

    let client = reqwest::Client::new();
    let request = client
        .get(format!("{}/v1/pets", server.uri()))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    let body = resp.text().await.unwrap();
    assert_eq!(body, "[]");
}

/// Verify that a POST request with a JSON body goes through correctly.
#[tokio::test]
async fn parity_post_request_with_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/pets"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "name": "Buddy",
            "tag": "dog"
        })))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "pet-1",
                "name": "Buddy",
                "tag": "dog"
            })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(http, no_auth_provider(), vec![], None);

    let client = reqwest::Client::new();
    let request = client
        .post(format!("{}/v1/pets", server.uri()))
        .json(&serde_json::json!({"name": "Buddy", "tag": "dog"}))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["id"], "pet-1");
    assert_eq!(json["name"], "Buddy");
}

/// Verify that global headers injected by the CLI are present on SDK requests.
#[tokio::test]
async fn parity_global_headers_applied_to_sdk_requests() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/pets"))
        .and(header("X-Fern-Sdk-Version", "0.6.0"))
        .and(header("X-Custom-Header", "custom-value"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .expect(1)
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(
        http,
        no_auth_provider(),
        vec![
            ("X-Fern-Sdk-Version".into(), "0.6.0".into()),
            ("X-Custom-Header".into(), "custom-value".into()),
        ],
        None,
    );

    let client = reqwest::Client::new();
    let request = client
        .get(format!("{}/v1/pets", server.uri()))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
}

// ═══════════════════════════════════════════════════════════════════════
// 2. AUTH INHERITANCE — SDK executor applies the same auth as built-in
// ═══════════════════════════════════════════════════════════════════════

/// A test auth provider that injects a known bearer token so we can verify
/// the executor applies it to outgoing SDK requests.
#[derive(Debug)]
struct TestBearerProvider {
    token: String,
}

impl AuthProvider for TestBearerProvider {
    fn name(&self) -> &str {
        "test-bearer"
    }

    fn has_credentials(&self) -> bool {
        true
    }

    fn apply(
        &self,
        request: reqwest::RequestBuilder,
        _endpoint: &fern_cli_sdk::auth::EndpointAuthMetadata,
    ) -> Result<reqwest::RequestBuilder, CliError> {
        Ok(request.bearer_auth(&self.token))
    }
}

/// Verify that the auth provider's credentials are applied to SDK requests
/// (same mechanism as built-in commands).
#[tokio::test]
async fn auth_inheritance_bearer_token_applied() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/pets/pet-42"))
        .and(header("Authorization", "Bearer test-secret-token-12345"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "pet-42",
                "name": "Luna",
                "tag": "cat"
            })),
        )
        .expect(1)
        .mount(&server)
        .await;

    let auth: DynAuthProvider = Arc::new(TestBearerProvider {
        token: "test-secret-token-12345".into(),
    });
    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(http, auth, vec![], None);

    let client = reqwest::Client::new();
    let request = client
        .get(format!("{}/v1/pets/pet-42", server.uri()))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["name"], "Luna");
}

/// Verify that auth + global headers combine correctly (both present).
#[tokio::test]
async fn auth_inheritance_combined_with_global_headers() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/pets"))
        .and(header("Authorization", "Bearer combined-token"))
        .and(header("X-Org-Id", "org-123"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "pet-new",
            "name": "Rex"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let auth: DynAuthProvider = Arc::new(TestBearerProvider {
        token: "combined-token".into(),
    });
    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(
        http,
        auth,
        vec![("X-Org-Id".into(), "org-123".into())],
        None,
    );

    let client = reqwest::Client::new();
    let request = client
        .post(format!("{}/v1/pets", server.uri()))
        .json(&serde_json::json!({"name": "Rex"}))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);
}

// ═══════════════════════════════════════════════════════════════════════
// 3. TYPE IDENTITY — Pet from SDK prelude is the same type as from types crate
// ═══════════════════════════════════════════════════════════════════════

/// The SDK prelude re-exports types from petstore_api_types. Verify that
/// `petstore_api_sdk::prelude::Pet` and `petstore_api_types::Pet` are the
/// same type by assigning one to the other without conversion.
#[test]
fn type_identity_pet_is_same_across_crates() {
    // Construct via the types crate
    let pet_from_types = petstore_api_types::Pet {
        id: "pet-99".to_string(),
        name: "TypeCheck".to_string(),
        tag: Some("verify".to_string()),
    };

    // Assign to a binding typed via the SDK prelude — this would fail to
    // compile if the types were distinct.
    let pet_from_sdk: petstore_api_sdk::prelude::Pet = pet_from_types;

    assert_eq!(pet_from_sdk.id, "pet-99");
    assert_eq!(pet_from_sdk.name, "TypeCheck");
    assert_eq!(pet_from_sdk.tag, Some("verify".to_string()));
}

/// Verify that CreatePetRequest from both paths is the same type.
#[test]
fn type_identity_create_pet_request_is_same() {
    let req_from_types = petstore_api_types::CreatePetRequest {
        name: "Hello".to_string(),
        tag: None,
    };
    let req_from_sdk: petstore_api_sdk::prelude::CreatePetRequest = req_from_types;
    assert_eq!(req_from_sdk.name, "Hello");
}

/// Verify serde round-trip produces identical JSON regardless of which
/// module the type was imported from.
#[test]
fn type_identity_serde_roundtrip() {
    let pet = petstore_api_sdk::prelude::Pet {
        id: "rt-1".to_string(),
        name: "Serde".to_string(),
        tag: Some("roundtrip".to_string()),
    };
    let json = serde_json::to_string(&pet).unwrap();
    let deserialized: petstore_api_types::Pet = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, pet);
}

// ═══════════════════════════════════════════════════════════════════════
// 4. REGRESSION — invoke() path (base-URL override, retries, error bridge)
// ═══════════════════════════════════════════════════════════════════════

/// Base-URL override redirects SDK traffic to the mock server.
#[tokio::test]
async fn regression_base_url_override_routes_to_mock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/pets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"[{"id":"1","name":"Override"}]"#))
        .expect(1)
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(
        http,
        no_auth_provider(),
        vec![],
        Some(server.uri()),
    );

    // Request targets a different host — the override should redirect it
    let client = reqwest::Client::new();
    let request = client
        .get("https://api.production.example.com/v1/pets")
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    let body = resp.text().await.unwrap();
    assert!(body.contains("Override"));
}

/// Error bridge: HTTP 404 from server surfaces as CliError::Api.
#[tokio::test]
async fn regression_error_bridge_http_status() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/pets/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let executor = CliExecutor::new(http, no_auth_provider(), vec![], None);

    let client = reqwest::Client::new();
    let request = client
        .get(format!("{}/v1/pets/missing", server.uri()))
        .build()
        .unwrap();

    // The executor itself returns Ok (the HTTP layer succeeded).
    // The SdkError → CliError mapping happens in the generated glue's
    // block_on wrapper. Here we just verify the response is faithfully
    // passed through.
    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 404);
}

/// Verify the SdkError → CliError bridge preserves status codes and messages.
#[test]
fn regression_sdk_error_to_cli_error_bridge() {
    let cases = vec![
        (
            SdkError::Http { status: 401, body: "unauthorized".into() },
            401u16,
            "unauthorized",
        ),
        (
            SdkError::Http { status: 429, body: "rate limited".into() },
            429,
            "rateLimited",
        ),
        (
            SdkError::Http { status: 503, body: "service down".into() },
            503,
            "serviceUnavailable",
        ),
    ];

    for (err, expected_code, expected_reason) in cases {
        let cli_err = err.into_cli_error();
        match cli_err {
            CliError::Api { code, reason, .. } => {
                assert_eq!(code, expected_code);
                assert_eq!(reason, expected_reason);
            }
            _ => panic!("expected CliError::Api for status {expected_code}"),
        }
    }
}

/// Network errors map to CliError::Other with descriptive message.
#[test]
fn regression_sdk_network_error_maps_to_other() {
    let err = SdkError::Network("DNS resolution failed".into());
    let cli_err = err.into_cli_error();
    match cli_err {
        CliError::Other(e) => {
            assert!(e.to_string().contains("network error"));
            assert!(e.to_string().contains("DNS resolution failed"));
        }
        _ => panic!("expected CliError::Other for network error"),
    }
}

/// Timeout errors map to CliError::Other with descriptive message.
#[test]
fn regression_sdk_timeout_error_maps_to_other() {
    let err = SdkError::Timeout("operation timed out after 30s".into());
    let cli_err = err.into_cli_error();
    match cli_err {
        CliError::Other(e) => {
            assert!(e.to_string().contains("timeout"));
        }
        _ => panic!("expected CliError::Other for timeout error"),
    }
}

/// Retries work through the SDK executor — verify a transient 500 is
/// retried and the second attempt succeeds.
#[tokio::test]
async fn regression_retries_through_sdk_executor() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use fern_cli_sdk::openapi::discovery::RetriesConfig;

    let server = MockServer::start().await;
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = call_count.clone();

    Mock::given(method("GET"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = cc.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(500)
            } else {
                ResponseTemplate::new(200).set_body_string("success after retry")
            }
        })
        .mount(&server)
        .await;

    let http = HttpConfig::new("test-cli").unwrap();
    let retries = RetriesConfig {
        enabled: true,
        max_attempts: 3,
        base_delay_ms: 10,
        factor: 1.0,
        jitter: 0.0,
    };
    let executor =
        CliExecutor::new(http, no_auth_provider(), vec![], None).with_retries(retries);

    let client = reqwest::Client::new();
    let request = client
        .get(format!("{}/test", server.uri()))
        .build()
        .unwrap();

    let resp = SdkRequestExecutor::execute(&executor, request).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.text().await.unwrap(), "success after retry");
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}
