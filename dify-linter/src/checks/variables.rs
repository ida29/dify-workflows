use crate::types::{ConversationVariable, LintContext, LintError, Node};
use regex::Regex;
use serde_json::Value;

/// Check conversation variables have required fields
pub fn check_conversation_variables(vars: &[ConversationVariable]) -> Vec<LintError> {
    let mut errors = Vec::new();

    for cv in vars {
        let name = cv.name.as_deref().unwrap_or("unknown");

        if cv.value_type.is_none() {
            errors.push(LintError::error_with_hint(
                "",
                &format!("cv:{}", name),
                "Missing 'value_type' in conversation variable",
                "Add: value_type: string",
            ));
        }

        if cv.value.is_none() {
            errors.push(LintError::error_with_hint(
                "",
                &format!("cv:{}", name),
                "Missing 'value' in conversation variable",
                "Add: value: ''",
            ));
        }
    }

    errors
}

/// Check all variable references in prompts
pub fn check_variable_references(nodes: &[Node], ctx: &LintContext) -> Vec<LintError> {
    let mut errors = Vec::new();
    let re = Regex::new(r"\{\{#([^#]+)#\}\}").unwrap();

    for node in nodes {
        let node_id = node.id.as_deref().unwrap_or("");
        let node_data = match &node.data {
            Some(d) => d,
            None => continue,
        };

        let node_type = node_data.node_type.as_deref().unwrap_or("");
        let node_title = node_data.title.as_deref().unwrap_or(node_type);

        // Check prompt_template for LLM nodes
        if node_type == "llm" {
            if let Some(prompt) = &node_data.prompt_template {
                errors.extend(check_prompt_references(node_id, node_title, prompt, ctx, &re));
            }
        }
    }

    errors
}

/// Check variable references in prompt template
fn check_prompt_references(
    node_id: &str,
    node_title: &str,
    prompt: &Value,
    ctx: &LintContext,
    re: &Regex,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    match prompt {
        Value::Object(obj) => {
            if let Some(Value::String(text)) = obj.get("text") {
                errors.extend(check_text_references(node_id, node_title, text, ctx, re));
            }
        }
        Value::Array(arr) => {
            for item in arr {
                if let Value::Object(obj) = item {
                    if let Some(Value::String(text)) = obj.get("text") {
                        errors.extend(check_text_references(node_id, node_title, text, ctx, re));
                    }
                }
            }
        }
        _ => {}
    }

    errors
}

/// Check variable references in text
fn check_text_references(
    node_id: &str,
    node_title: &str,
    text: &str,
    ctx: &LintContext,
    re: &Regex,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    for cap in re.captures_iter(text) {
        if let Some(ref_str) = cap.get(1) {
            let parts: Vec<&str> = ref_str.as_str().split('.').collect();
            if let Some(&ref_source) = parts.first() {
                // Valid system references
                if ref_source == "sys" || ref_source == "conversation" {
                    continue;
                }

                // Check if node exists
                if !ctx.node_exists(ref_source) {
                    errors.push(LintError::error_with_hint(
                        node_id,
                        node_title,
                        &format!("References non-existent node: {}", ref_source),
                        "Check node ID or use sys.query / conversation.var",
                    ));
                }
            }
        }
    }

    errors
}
