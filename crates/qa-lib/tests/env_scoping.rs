use greentic_qa_lib::{
    I18nConfig, NoopSecretsSink, QaLibError, SecretRef, SecretsSink, WizardDriver, WizardFrontend,
    WizardRunConfig,
};
use serde_json::{Value, json};

fn minimal_spec() -> Value {
    json!({
        "id": "form.env_scoping",
        "version": "0.1.0",
        "questions": []
    })
}

fn make_config(env_id: &str) -> WizardRunConfig {
    WizardRunConfig {
        spec_json: minimal_spec().to_string(),
        initial_answers_json: None,
        frontend: WizardFrontend::JsonUi,
        i18n: I18nConfig::default(),
        verbose: false,
        env_id: env_id.into(),
    }
}

#[test]
fn driver_round_trips_env_id() {
    let driver = WizardDriver::new(make_config("staging")).expect("driver constructs");
    assert_eq!(driver.env_id(), "staging");
}

#[test]
fn noop_secrets_sink_keeps_value_inline() {
    let sink = NoopSecretsSink;
    let result = sink
        .try_route("local", "db_password", &json!("hunter2"))
        .expect("noop sink never fails");
    assert_eq!(result, None);
}

#[test]
fn custom_secrets_sink_can_substitute_a_ref() {
    struct RoutingSink;
    impl SecretsSink for RoutingSink {
        fn try_route(
            &self,
            env_id: &str,
            question_id: &str,
            _value: &Value,
        ) -> Result<Option<SecretRef>, QaLibError> {
            Ok(Some(SecretRef {
                uri: format!("secret://{env_id}/{question_id}"),
            }))
        }
    }

    let routed = RoutingSink
        .try_route("local", "db_password", &json!("hunter2"))
        .expect("routing sink returns a ref")
        .expect("ref was substituted");
    assert_eq!(routed.uri, "secret://local/db_password");
}
