#[cfg(target_arch = "wasm32")]
use std::collections::BTreeMap;

#[cfg(target_arch = "wasm32")]
use greentic_interfaces_guest::component_v0_6::node;
#[cfg(target_arch = "wasm32")]
use greentic_interfaces_guest::component_v0_6::{component_i18n, component_qa};
#[cfg(target_arch = "wasm32")]
use greentic_types::cbor::canonical;
#[cfg(target_arch = "wasm32")]
use greentic_types::schemas::common::schema_ir::{AdditionalProperties, SchemaIr};
#[cfg(target_arch = "wasm32")]
use serde_cbor::Value as CborValue;
#[cfg(target_arch = "wasm32")]
use serde_json::json;

pub mod i18n;
pub mod i18n_bundle;
pub mod qa;
pub use qa::{
    apply_store, describe, get_answer_schema, get_example_answers, next, next_with_ctx,
    render_card, render_json_ui, render_text, submit_all, submit_patch, validate_answers,
};

const COMPONENT_NAME: &str = "component-qa";
const COMPONENT_ORG: &str = "ai.greentic";
const COMPONENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_arch = "wasm32")]
#[used]
#[unsafe(link_section = ".greentic.wasi")]
static WASI_TARGET_MARKER: [u8; 13] = *b"wasm32-wasip2";

#[cfg(target_arch = "wasm32")]
struct Component;

#[cfg(target_arch = "wasm32")]
impl node::Guest for Component {
    fn describe() -> node::ComponentDescriptor {
        let input_schema_cbor = message_input_schema_cbor();
        let output_schema_cbor = message_output_schema_cbor();
        let setup_apply_input_schema_cbor = setup_apply_input_schema_cbor();
        let setup_apply_output_schema_cbor = setup_apply_output_schema_cbor();
        node::ComponentDescriptor {
            name: COMPONENT_NAME.to_string(),
            version: COMPONENT_VERSION.to_string(),
            summary: Some("Greentic QA component".to_string()),
            capabilities: Vec::new(),
            ops: vec![
                node::Op {
                    name: "run".to_string(),
                    summary: Some("Compatibility alias for handle_message".to_string()),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(input_schema_cbor.clone()),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(output_schema_cbor.clone()),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
                node::Op {
                    name: "handle_message".to_string(),
                    summary: Some("Handle a single message input".to_string()),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(input_schema_cbor.clone()),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(output_schema_cbor.clone()),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
                node::Op {
                    name: "qa-spec".to_string(),
                    summary: Some("Return QA spec (CBOR) for a requested mode".to_string()),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(
                            setup_apply_input_schema_cbor.clone(),
                        ),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(bootstrap_qa_spec_schema_cbor()),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
                node::Op {
                    name: "apply-answers".to_string(),
                    summary: Some(
                        "Apply QA answers and optionally return config override".to_string(),
                    ),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(
                            setup_apply_input_schema_cbor.clone(),
                        ),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(
                            setup_apply_output_schema_cbor.clone(),
                        ),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
                node::Op {
                    name: "setup.apply_answers".to_string(),
                    summary: Some("Apply setup wizard answers and return config CBOR".to_string()),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(setup_apply_input_schema_cbor),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(setup_apply_output_schema_cbor),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
                node::Op {
                    name: "i18n-keys".to_string(),
                    summary: Some("Return i18n keys referenced by QA/setup".to_string()),
                    input: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(input_schema_cbor),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    output: node::IoSchema {
                        schema: node::SchemaSource::InlineCbor(output_schema_cbor),
                        content_type: "application/cbor".to_string(),
                        schema_version: None,
                    },
                    examples: Vec::new(),
                },
            ],
            schemas: Vec::new(),
            setup: Some(node::SetupContract {
                qa_spec: node::SchemaSource::InlineCbor(bootstrap_qa_spec_cbor()),
                answers_schema: node::SchemaSource::InlineCbor(bootstrap_answers_schema_cbor()),
                examples: Vec::new(),
                outputs: vec![node::SetupOutput::ConfigOnly],
            }),
        }
    }

    fn invoke(
        operation: String,
        envelope: node::InvocationEnvelope,
    ) -> Result<node::InvocationResult, node::NodeError> {
        Ok(node::InvocationResult {
            ok: true,
            output_cbor: run_component_cbor(&operation, envelope.payload_cbor),
            output_metadata_cbor: None,
        })
    }
}

#[cfg(target_arch = "wasm32")]
impl component_qa::Guest for Component {
    fn qa_spec(mode: component_qa::QaMode) -> Vec<u8> {
        let mode = normalized_mode_from_qa_mode(mode);
        let payload = qa::qa_spec_json(mode, &json!({ "form_id": "component-qa" }));
        encode_cbor(&payload)
    }

    fn apply_answers(
        mode: component_qa::QaMode,
        current_config: Vec<u8>,
        answers: Vec<u8>,
    ) -> Vec<u8> {
        let payload = json!({
            "mode": normalized_mode_from_qa_mode(mode).as_str(),
            "current_config": canonical::from_cbor(&current_config).unwrap_or_else(|_| json!({})),
            "answers": canonical::from_cbor(&answers).unwrap_or_else(|_| json!({})),
        });
        let result = qa::apply_answers(normalized_mode_from_qa_mode(mode), &payload);
        let config = result
            .get("config")
            .cloned()
            .unwrap_or_else(|| payload["current_config"].clone());
        encode_cbor(&config)
    }
}

#[cfg(target_arch = "wasm32")]
impl component_i18n::Guest for Component {
    fn i18n_keys() -> Vec<String> {
        qa::i18n_keys()
    }
}

#[cfg(target_arch = "wasm32")]
greentic_interfaces_guest::export_component_v060!(
    Component,
    component_qa: Component,
    component_i18n: Component,
);

pub fn describe_payload() -> String {
    serde_json::json!({
        "component": {
            "name": COMPONENT_NAME,
            "org": COMPONENT_ORG,
            "version": COMPONENT_VERSION,
            "world": "greentic:component/component@0.6.0",
            "self_describing": true
        }
    })
    .to_string()
}

pub fn handle_message(operation: &str, input: &str) -> String {
    format!("{COMPONENT_NAME}::{operation} => {}", input.trim())
}

#[cfg(target_arch = "wasm32")]
fn encode_cbor<T: serde::Serialize>(value: &T) -> Vec<u8> {
    canonical::to_canonical_cbor_allow_floats(value).expect("encode cbor")
}

#[cfg(target_arch = "wasm32")]
fn parse_payload(input: &[u8]) -> serde_json::Value {
    if let Ok(value) = canonical::from_cbor(input) {
        return value;
    }
    serde_json::from_slice(input).unwrap_or_else(|_| serde_json::json!({}))
}

#[cfg(target_arch = "wasm32")]
fn message_input_schema() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([(
            "input".to_string(),
            SchemaIr::String {
                min_len: Some(0),
                max_len: None,
                regex: None,
                format: None,
            },
        )]),
        required: vec!["input".to_string()],
        additional: AdditionalProperties::Allow,
    }
}

#[cfg(target_arch = "wasm32")]
fn message_output_schema() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([(
            "message".to_string(),
            SchemaIr::String {
                min_len: Some(0),
                max_len: None,
                regex: None,
                format: None,
            },
        )]),
        required: vec!["message".to_string()],
        additional: AdditionalProperties::Allow,
    }
}

#[cfg(target_arch = "wasm32")]
fn bootstrap_answers_schema() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([(
            "qa_form_asset_path".to_string(),
            SchemaIr::String {
                min_len: Some(1),
                max_len: None,
                regex: None,
                format: None,
            },
        )]),
        required: vec!["qa_form_asset_path".to_string()],
        additional: AdditionalProperties::Allow,
    }
}

#[cfg(target_arch = "wasm32")]
fn setup_apply_input_schema() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([
            (
                "mode".to_string(),
                SchemaIr::String {
                    min_len: Some(0),
                    max_len: None,
                    regex: None,
                    format: None,
                },
            ),
            (
                "current_config_cbor".to_string(),
                SchemaIr::String {
                    min_len: Some(0),
                    max_len: None,
                    regex: None,
                    format: None,
                },
            ),
            (
                "answers_cbor".to_string(),
                SchemaIr::String {
                    min_len: Some(0),
                    max_len: None,
                    regex: None,
                    format: None,
                },
            ),
        ]),
        required: Vec::new(),
        additional: AdditionalProperties::Allow,
    }
}

#[cfg(target_arch = "wasm32")]
fn setup_apply_output_schema() -> SchemaIr {
    SchemaIr::Object {
        properties: BTreeMap::from([(
            "qa_form_asset_path".to_string(),
            SchemaIr::String {
                min_len: Some(1),
                max_len: None,
                regex: None,
                format: None,
            },
        )]),
        required: Vec::new(),
        additional: AdditionalProperties::Allow,
    }
}

#[cfg(target_arch = "wasm32")]
fn message_input_schema_cbor() -> Vec<u8> {
    encode_cbor(&message_input_schema())
}

#[cfg(target_arch = "wasm32")]
fn message_output_schema_cbor() -> Vec<u8> {
    encode_cbor(&message_output_schema())
}

#[cfg(target_arch = "wasm32")]
fn bootstrap_answers_schema_cbor() -> Vec<u8> {
    encode_cbor(&bootstrap_answers_schema())
}

#[cfg(target_arch = "wasm32")]
fn setup_apply_input_schema_cbor() -> Vec<u8> {
    encode_cbor(&setup_apply_input_schema())
}

#[cfg(target_arch = "wasm32")]
fn setup_apply_output_schema_cbor() -> Vec<u8> {
    encode_cbor(&setup_apply_output_schema())
}

#[cfg(target_arch = "wasm32")]
fn bootstrap_qa_spec_value() -> serde_json::Value {
    qa::qa_spec_json(
        qa::NormalizedMode::Setup,
        &json!({ "form_id": "component-qa" }),
    )
}

#[cfg(target_arch = "wasm32")]
fn bootstrap_qa_spec_cbor() -> Vec<u8> {
    encode_cbor(&bootstrap_qa_spec_value())
}

#[cfg(target_arch = "wasm32")]
fn bootstrap_qa_spec_schema_cbor() -> Vec<u8> {
    encode_cbor(&SchemaIr::Object {
        properties: BTreeMap::new(),
        required: Vec::new(),
        additional: AdditionalProperties::Allow,
    })
}

#[cfg(target_arch = "wasm32")]
fn normalized_mode(payload: &serde_json::Value) -> qa::NormalizedMode {
    let mode = payload
        .get("mode")
        .and_then(|v| v.as_str())
        .or_else(|| payload.get("operation").and_then(|v| v.as_str()))
        .unwrap_or("setup");
    qa::normalize_mode(mode).unwrap_or(qa::NormalizedMode::Setup)
}

#[cfg(target_arch = "wasm32")]
fn normalized_mode_from_qa_mode(mode: component_qa::QaMode) -> qa::NormalizedMode {
    match mode {
        component_qa::QaMode::Default | component_qa::QaMode::Setup => qa::NormalizedMode::Setup,
        component_qa::QaMode::Update => qa::NormalizedMode::Update,
        component_qa::QaMode::Remove => qa::NormalizedMode::Remove,
    }
}

#[cfg(target_arch = "wasm32")]
fn cbor_map_get<'a>(map: &'a BTreeMap<CborValue, CborValue>, key: &str) -> Option<&'a CborValue> {
    map.get(&CborValue::Text(key.to_string()))
}

#[cfg(target_arch = "wasm32")]
fn decode_nested_cbor_json(value: Option<&CborValue>) -> serde_json::Value {
    match value {
        Some(CborValue::Bytes(bytes)) => canonical::from_cbor(bytes).unwrap_or_else(|_| json!({})),
        _ => json!({}),
    }
}

#[cfg(target_arch = "wasm32")]
fn parse_setup_apply_payload(input: &[u8]) -> serde_json::Value {
    let decoded = serde_cbor::from_slice::<CborValue>(input)
        .unwrap_or_else(|_| CborValue::Map(BTreeMap::new()));
    let CborValue::Map(entries) = decoded else {
        return json!({});
    };

    let mode = match cbor_map_get(&entries, "mode") {
        Some(CborValue::Text(mode)) => mode.as_str(),
        _ => "setup",
    };

    json!({
        "mode": mode,
        "current_config": decode_nested_cbor_json(cbor_map_get(&entries, "current_config_cbor")),
        "answers": decode_nested_cbor_json(cbor_map_get(&entries, "answers_cbor")),
    })
}

#[cfg(target_arch = "wasm32")]
fn run_setup_apply_cbor(input: &[u8]) -> Vec<u8> {
    let payload = parse_setup_apply_payload(input);
    let mode = normalized_mode(&payload);
    let result = qa::apply_answers(mode, &payload);
    let config = result.get("config").cloned().unwrap_or_else(|| {
        payload
            .get("current_config")
            .cloned()
            .unwrap_or_else(|| json!({}))
    });
    encode_cbor(&config)
}

#[cfg(target_arch = "wasm32")]
fn run_component_cbor(operation: &str, input: Vec<u8>) -> Vec<u8> {
    if operation == "setup.apply_answers" {
        return run_setup_apply_cbor(&input);
    }

    let value = parse_payload(&input);
    let output = match operation {
        "qa-spec" => {
            let mode = normalized_mode(&value);
            qa::qa_spec_json(mode, &value)
        }
        "apply-answers" => {
            let mode = normalized_mode(&value);
            qa::apply_answers(mode, &value)
        }
        "i18n-keys" => serde_json::Value::Array(
            qa::i18n_keys()
                .into_iter()
                .map(serde_json::Value::String)
                .collect(),
        ),
        _ => {
            let op_name = value
                .get("operation")
                .and_then(|v| v.as_str())
                .unwrap_or(operation);
            let input_text = value
                .get("input")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| value.to_string());
            serde_json::json!({
                "message": handle_message(op_name, &input_text)
            })
        }
    };

    encode_cbor(&output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_payload_is_json() {
        let payload = describe_payload();
        let json: serde_json::Value = serde_json::from_str(&payload).expect("valid json");
        assert_eq!(json["component"]["name"], "component-qa");
    }

    #[test]
    fn handle_message_round_trips() {
        let body = handle_message("handle", "demo");
        assert!(body.contains("demo"));
    }
}
