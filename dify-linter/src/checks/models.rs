use crate::types::{LintError, Node};

const VALID_MODELS: &[&str] = &[
    "ai21",
    "amazon nova",
    "claude",
    "gpt-4",
    "gpt-3.5-turbo",
    "claude-3-sonnet",
    "claude-3-opus",
    "claude-3-haiku",
];

/// Check model configurations are valid
pub fn check_model_config(nodes: &[Node]) -> Vec<LintError> {
    let mut errors = Vec::new();

    for node in nodes {
        let node_id = node.id.as_deref().unwrap_or("");
        let node_data = match &node.data {
            Some(d) => d,
            None => continue,
        };

        let node_type = node_data.node_type.as_deref().unwrap_or("");
        let node_title = node_data.title.as_deref().unwrap_or(node_type);

        // Check LLM and question-classifier nodes
        if node_type == "llm" || node_type == "question-classifier" {
            if let Some(model) = &node_data.model {
                if let Some(model_name) = &model.name {
                    let model_lower = model_name.to_lowercase();
                    let is_known = VALID_MODELS
                        .iter()
                        .any(|m| model_lower.contains(&m.to_lowercase()));

                    if !is_known {
                        errors.push(LintError::warning(
                            node_id,
                            node_title,
                            &format!("Unknown model: {}", model_name),
                        ));
                    }
                }
            }
        }
    }

    errors
}
