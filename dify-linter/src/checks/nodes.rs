use crate::types::{LintContext, LintError, Node, NodeData};
use serde_json::Value;
use std::collections::HashSet;

/// Check all nodes for issues
pub fn check_nodes(nodes: &[Node], ctx: &LintContext) -> Vec<LintError> {
    let mut errors = Vec::new();

    for node in nodes {
        let node_id = node.id.as_deref().unwrap_or("");
        let node_data = match &node.data {
            Some(d) => d,
            None => {
                errors.push(LintError::error(node_id, "unknown", "Node missing 'data'"));
                continue;
            }
        };

        let node_type = node_data.node_type.as_deref().unwrap_or("");
        let node_title = node_data.title.as_deref().unwrap_or(node_type);

        // Check required fields
        if node_id.is_empty() {
            errors.push(LintError::error("", node_title, "Node missing 'id'"));
        }

        if node_type.is_empty() {
            errors.push(LintError::error(node_id, node_title, "Node missing 'type'"));
        }

        // Type-specific checks
        match node_type {
            "llm" => errors.extend(check_llm_node(node_id, node_title, node_data)),
            "if-else" => errors.extend(check_if_else_node(node_id, node_title, node_data, ctx)),
            "question-classifier" => {
                errors.extend(check_question_classifier_node(node_id, node_title, node_data, ctx))
            }
            "variable-aggregator" => {
                errors.extend(check_variable_aggregator(node_id, node_title, node_data, ctx))
            }
            "answer" => errors.extend(check_answer_node(node_id, node_title, node_data, ctx)),
            "variable-assigner" => {
                errors.extend(check_variable_assigner(node_id, node_title, node_data))
            }
            "assigner" => {
                errors.extend(check_assigner_v2(node_id, node_title, node_data))
            }
            "code" => errors.extend(check_code_node(node_id, node_title, node_data, ctx)),
            "tool" => errors.extend(check_tool_node(node_id, node_title, node_data)),
            "iteration" => errors.extend(check_iteration_node(node_id, node_title, node_data, ctx)),
            "knowledge-retrieval" => {
                errors.extend(check_knowledge_retrieval_node(node_id, node_title, node_data))
            }
            _ => {}
        }
    }

    errors
}

/// Check LLM node configuration
fn check_llm_node(node_id: &str, node_title: &str, data: &NodeData) -> Vec<LintError> {
    let mut errors = Vec::new();

    let model = match &data.model {
        Some(m) => m,
        None => {
            errors.push(LintError::error(
                node_id,
                node_title,
                "LLM node missing 'model' configuration",
            ));
            return errors;
        }
    };

    if model.name.is_none() {
        errors.push(LintError::error(
            node_id,
            node_title,
            "LLM model missing 'name'",
        ));
    }

    if model.provider.is_none() {
        errors.push(LintError::error(
            node_id,
            node_title,
            "LLM model missing 'provider'",
        ));
    }

    // Check prompt_template
    match &data.prompt_template {
        None => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                "LLM node has empty prompt_template",
            ));
        }
        Some(Value::Array(arr)) if arr.is_empty() => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                "LLM node has empty prompt_template",
            ));
        }
        _ => {}
    }

    errors
}

/// Check IF/ELSE node conditions
fn check_if_else_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check for conditions in either legacy format (conditions) or new format (cases)
    let has_conditions = data.conditions.as_ref().map(|c| !c.is_empty()).unwrap_or(false);
    let has_cases = data.cases.as_ref().map(|c| !c.is_empty()).unwrap_or(false);

    if !has_conditions && !has_cases {
        errors.push(LintError::error(
            node_id,
            node_title,
            "IF/ELSE node has no conditions",
        ));
        return errors;
    }

    // Helper function to check conditions
    let check_condition = |cond: &crate::types::Condition, errors: &mut Vec<LintError>| {
        if let Some(var_selector) = &cond.variable_selector {
            if let Some(ref_node_id) = var_selector.first() {
                // Check if referenced node exists (sys and conversation are special keywords)
                if !ctx.node_exists(ref_node_id) && ref_node_id != "sys" && ref_node_id != "conversation" {
                    errors.push(LintError::error_with_hint(
                        node_id,
                        node_title,
                        &format!("IF/ELSE references non-existent node: {}", ref_node_id),
                        "Use sys.query or valid node ID",
                    ));
                }

                // Check if referencing start node with empty variables
                if let Some(ref_node) = ctx.get_node(ref_node_id) {
                    if let Some(ref_data) = &ref_node.data {
                        if ref_data.node_type.as_deref() == Some("start") {
                            let has_vars = ref_data
                                .variables
                                .as_ref()
                                .map(|v| match v {
                                    Value::Array(arr) => !arr.is_empty(),
                                    _ => false,
                                })
                                .unwrap_or(false);

                            if !has_vars && var_selector.len() > 1 {
                                let var_name = &var_selector[1];
                                errors.push(LintError::error_with_hint(
                                    node_id,
                                    node_title,
                                    &format!(
                                        "References '{}' from start node, but start has no variables",
                                        var_name
                                    ),
                                    "Either add variables to start node or use sys.query",
                                ));
                            }
                        }
                    }
                }
            }
        }
    };

    // Check legacy conditions format
    if let Some(conditions) = &data.conditions {
        for cond_group in conditions {
            if let Some(conds) = &cond_group.conditions {
                for cond in conds {
                    check_condition(cond, &mut errors);
                }
            }
        }
    }

    // Check new cases format
    if let Some(cases) = &data.cases {
        for case in cases {
            if let Some(conds) = &case.conditions {
                for cond in conds {
                    check_condition(cond, &mut errors);
                }
            }
        }
    }

    errors
}

/// Check Question Classifier configuration
fn check_question_classifier_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    let classes = match &data.classes {
        Some(c) if !c.is_empty() => c,
        _ => {
            errors.push(LintError::error(
                node_id,
                node_title,
                "Question Classifier has no classes",
            ));
            return errors;
        }
    };

    // Check model
    if data.model.is_none() {
        errors.push(LintError::error(
            node_id,
            node_title,
            "Question Classifier missing 'model'",
        ));
    }

    // Check each class has an id and name
    let mut class_ids = HashSet::new();
    for cls in classes {
        match &cls.id {
            Some(id) => {
                class_ids.insert(id.clone());
            }
            None => {
                errors.push(LintError::error(node_id, node_title, "Class missing 'id'"));
            }
        }

        if cls.name.is_none() {
            let cls_id = cls.id.as_deref().unwrap_or("unknown");
            errors.push(LintError::error(
                node_id,
                node_title,
                &format!("Class {} missing 'name'", cls_id),
            ));
        }
    }

    // Check that edges exist for each class
    let edge_handles: HashSet<String> = ctx
        .edges
        .iter()
        .filter(|e| e.source.as_deref() == Some(node_id))
        .filter_map(|e| e.source_handle.clone())
        .collect();

    for cls_id in &class_ids {
        if !edge_handles.contains(cls_id) {
            errors.push(LintError::warning(
                node_id,
                node_title,
                &format!("Class '{}' has no outgoing edge - will go nowhere", cls_id),
            ));
        }
    }

    // Check query_variable_selector
    if let Some(query_var) = &data.query_variable_selector {
        if query_var.len() >= 2 {
            let source = &query_var[0];
            let var = &query_var[1];
            if source == "sys" && var == "query" {
                // Valid
            } else if !ctx.node_exists(source) {
                errors.push(LintError::error_with_hint(
                    node_id,
                    node_title,
                    &format!(
                        "query_variable_selector references non-existent node: {}",
                        source
                    ),
                    "Use sys.query or valid node ID",
                ));
            } else {
                // Check if referencing start node with empty variables
                if let Some(ref_node) = ctx.get_node(source) {
                    if let Some(ref_data) = &ref_node.data {
                        if ref_data.node_type.as_deref() == Some("start") {
                            let has_vars = ref_data
                                .variables
                                .as_ref()
                                .map(|v| match v {
                                    Value::Array(arr) => !arr.is_empty(),
                                    _ => false,
                                })
                                .unwrap_or(false);

                            if !has_vars {
                                errors.push(LintError::error_with_hint(
                                    node_id,
                                    node_title,
                                    &format!(
                                        "query_variable_selector references '{}' from start node, but start has no variables",
                                        var
                                    ),
                                    "Either add variables to start node or use sys.query",
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    errors
}

/// Check Variable Aggregator configuration
fn check_variable_aggregator(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    let variables = match &data.variables {
        Some(Value::Array(arr)) if !arr.is_empty() => arr,
        _ => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                "Variable Aggregator has no variables",
            ));
            return errors;
        }
    };

    // Check each referenced variable exists
    for var in variables {
        if let Value::Array(var_arr) = var {
            if let Some(Value::String(ref_node_id)) = var_arr.first() {
                if !ctx.node_exists(ref_node_id) {
                    errors.push(LintError::error(
                        node_id,
                        node_title,
                        &format!("References non-existent node: {}", ref_node_id),
                    ));
                }
            }
        }
    }

    errors
}

/// Check Variable Assigner configuration
fn check_variable_assigner(node_id: &str, node_title: &str, data: &NodeData) -> Vec<LintError> {
    let mut errors = Vec::new();

    // WARNING: Variable Assigner may cause client-side errors in Dify 1.10.x
    // See: https://github.com/langgenius/dify/issues/XXXX
    errors.push(LintError::warning_with_hint(
        node_id,
        node_title,
        "Variable Assigner node may cause 'e.slice is not a function' error in Dify 1.10.x",
        "Consider using Variable Aggregator instead, or test carefully after import",
    ));

    // Check variables have write_mode
    if let Some(Value::Array(vars)) = &data.variables {
        for var in vars {
            if let Value::Object(obj) = var {
                if !obj.contains_key("write_mode") {
                    errors.push(LintError::error_with_hint(
                        node_id,
                        node_title,
                        "Variable Assigner variable missing 'write_mode'",
                        "Add: write_mode: 'over-write'",
                    ));
                }
            }
        }
    }

    errors
}

/// Check Assigner V2 node configuration (new format)
fn check_assigner_v2(node_id: &str, node_title: &str, data: &NodeData) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check version field
    let version = data.extra.get("version").and_then(|v| v.as_str());
    if version != Some("2") {
        errors.push(LintError::warning_with_hint(
            node_id,
            node_title,
            "Assigner node should have version: \"2\"",
            "Add: version: \"2\"",
        ));
    }

    // Check items array exists
    let items = data.extra.get("items").and_then(|v| v.as_array());
    match items {
        None => {
            errors.push(LintError::error_with_hint(
                node_id,
                node_title,
                "Assigner V2 node missing 'items' array",
                "Add items array with variable_selector, input_type, operation, value",
            ));
        }
        Some(items_arr) => {
            for (i, item) in items_arr.iter().enumerate() {
                if let Some(obj) = item.as_object() {
                    // Check required fields
                    if !obj.contains_key("variable_selector") {
                        errors.push(LintError::error_with_hint(
                            node_id,
                            node_title,
                            &format!("items[{}] missing 'variable_selector'", i),
                            "Add: variable_selector: [conversation, var_name]",
                        ));
                    }
                    if !obj.contains_key("operation") {
                        errors.push(LintError::error_with_hint(
                            node_id,
                            node_title,
                            &format!("items[{}] missing 'operation'", i),
                            "Add: operation: overwrite (or append, clear, etc.)",
                        ));
                    }
                }
            }
        }
    }

    errors
}

/// Check Answer node configuration
fn check_answer_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    let answer = match &data.answer {
        Some(a) if !a.is_empty() => a,
        _ => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                "Answer node has empty answer",
            ));
            return errors;
        }
    };

    // Check variable references in answer using regex
    let re = regex::Regex::new(r"\{\{#([^#]+)#\}\}").unwrap();
    for cap in re.captures_iter(answer) {
        if let Some(ref_str) = cap.get(1) {
            let parts: Vec<&str> = ref_str.as_str().split('.').collect();
            if let Some(&ref_node_id) = parts.first() {
                if !ctx.node_exists(ref_node_id)
                    && ref_node_id != "sys"
                    && ref_node_id != "conversation"
                {
                    errors.push(LintError::error(
                        node_id,
                        node_title,
                        &format!("Answer references non-existent node: {}", ref_node_id),
                    ));
                }
            }
        }
    }

    errors
}

/// Check Code node configuration
fn check_code_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check code field exists
    let code = data.extra.get("code").and_then(|v| v.as_str());
    if code.is_none() || code.map(|c| c.is_empty()).unwrap_or(true) {
        errors.push(LintError::error(
            node_id,
            node_title,
            "Code node missing 'code' field",
        ));
    }

    // Check code_language
    let lang = data.extra.get("code_language").and_then(|v| v.as_str());
    match lang {
        None => {
            errors.push(LintError::warning_with_hint(
                node_id,
                node_title,
                "Code node missing 'code_language'",
                "Add: code_language: python3",
            ));
        }
        Some(l) if l != "python3" && l != "javascript" => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                &format!("Unknown code_language: {} (expected python3 or javascript)", l),
            ));
        }
        _ => {}
    }

    // Check outputs
    let outputs = data.extra.get("outputs");
    if outputs.is_none() {
        errors.push(LintError::warning_with_hint(
            node_id,
            node_title,
            "Code node missing 'outputs' definition",
            "Add outputs with variable names and types",
        ));
    }

    // Check variable references in code
    if let Some(code_str) = code {
        let re = regex::Regex::new(r"\{\{#([^#]+)#\}\}").unwrap();
        for cap in re.captures_iter(code_str) {
            if let Some(ref_str) = cap.get(1) {
                let parts: Vec<&str> = ref_str.as_str().split('.').collect();
                if let Some(&ref_node_id) = parts.first() {
                    if !ctx.node_exists(ref_node_id)
                        && ref_node_id != "sys"
                        && ref_node_id != "conversation"
                    {
                        errors.push(LintError::error(
                            node_id,
                            node_title,
                            &format!("Code references non-existent node: {}", ref_node_id),
                        ));
                    }
                }
            }
        }
    }

    errors
}

/// Check Tool node configuration
fn check_tool_node(node_id: &str, node_title: &str, data: &NodeData) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check provider_id
    let provider_id = data.extra.get("provider_id").and_then(|v| v.as_str());
    if provider_id.is_none() || provider_id.map(|p| p.is_empty()).unwrap_or(true) {
        errors.push(LintError::error_with_hint(
            node_id,
            node_title,
            "Tool node missing 'provider_id'",
            "Add: provider_id: langgenius/tavily/tavily",
        ));
    }

    // Check tool_name
    let tool_name = data.extra.get("tool_name").and_then(|v| v.as_str());
    if tool_name.is_none() || tool_name.map(|t| t.is_empty()).unwrap_or(true) {
        errors.push(LintError::error_with_hint(
            node_id,
            node_title,
            "Tool node missing 'tool_name'",
            "Add: tool_name: tavily_search or tavily_extract",
        ));
    }

    // Validate known providers and tools
    if let (Some(provider), Some(tool)) = (provider_id, tool_name) {
        match provider {
            "langgenius/tavily/tavily" | "tavily" => {
                if tool != "tavily_search" && tool != "tavily_extract" {
                    errors.push(LintError::warning(
                        node_id,
                        node_title,
                        &format!("Unknown Tavily tool: {} (expected tavily_search or tavily_extract)", tool),
                    ));
                }
            }
            "json_process" => {
                // JSON process tool - valid
            }
            _ => {
                // Unknown provider - just a warning
                errors.push(LintError::warning(
                    node_id,
                    node_title,
                    &format!("Unknown tool provider: {}", provider),
                ));
            }
        }
    }

    errors
}

/// Check Iteration node configuration
fn check_iteration_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
    ctx: &LintContext,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check iterator_selector
    let iterator = data.extra.get("iterator_selector").and_then(|v| v.as_array());
    match iterator {
        None => {
            errors.push(LintError::error_with_hint(
                node_id,
                node_title,
                "Iteration node missing 'iterator_selector'",
                "Add: iterator_selector: [node_id, output_var]",
            ));
        }
        Some(arr) => {
            if let Some(source) = arr.first().and_then(|v| v.as_str()) {
                if !ctx.node_exists(source) && source != "sys" && source != "conversation" {
                    errors.push(LintError::error(
                        node_id,
                        node_title,
                        &format!("Iteration references non-existent node: {}", source),
                    ));
                }
            }
        }
    }

    // Check output_selector
    let output = data.extra.get("output_selector").and_then(|v| v.as_array());
    if output.is_none() {
        errors.push(LintError::warning_with_hint(
            node_id,
            node_title,
            "Iteration node missing 'output_selector'",
            "Add: output_selector: [inner_node_id, output_var]",
        ));
    }

    errors
}

/// Check Knowledge Retrieval node configuration
fn check_knowledge_retrieval_node(
    node_id: &str,
    node_title: &str,
    data: &NodeData,
) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check dataset_ids
    let dataset_ids = data.extra.get("dataset_ids").and_then(|v| v.as_array());
    if dataset_ids.is_none() || dataset_ids.map(|d| d.is_empty()).unwrap_or(true) {
        errors.push(LintError::error_with_hint(
            node_id,
            node_title,
            "Knowledge Retrieval node missing 'dataset_ids'",
            "Add dataset_ids array with knowledge base UUIDs",
        ));
    }

    // Check retrieval_mode
    let retrieval_mode = data.extra.get("retrieval_mode").and_then(|v| v.as_str());
    match retrieval_mode {
        None => {
            errors.push(LintError::warning_with_hint(
                node_id,
                node_title,
                "Knowledge Retrieval missing 'retrieval_mode'",
                "Add: retrieval_mode: single or multiple",
            ));
        }
        Some(mode) if mode != "single" && mode != "multiple" => {
            errors.push(LintError::warning(
                node_id,
                node_title,
                &format!("Unknown retrieval_mode: {} (expected single or multiple)", mode),
            ));
        }
        _ => {}
    }

    // Check for rerank model (can cause issues without OpenAI)
    if let Some(settings) = data.extra.get("single_retrieval_config") {
        if let Some(model) = settings.get("reranking_model") {
            if model.get("provider").and_then(|p| p.as_str()) == Some("openai") {
                errors.push(LintError::warning_with_hint(
                    node_id,
                    node_title,
                    "Rerank model uses OpenAI - may fail without OpenAI plugin",
                    "Consider using weighted_score mode instead of rerank",
                ));
            }
        }
    }

    if let Some(settings) = data.extra.get("multiple_retrieval_config") {
        if let Some(model) = settings.get("reranking_model") {
            if model.get("provider").and_then(|p| p.as_str()) == Some("openai") {
                errors.push(LintError::warning_with_hint(
                    node_id,
                    node_title,
                    "Rerank model uses OpenAI - may fail without OpenAI plugin",
                    "Consider using weighted_score mode instead of rerank",
                ));
            }
        }
    }

    errors
}
