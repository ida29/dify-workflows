use crate::types::{DifyDsl, LintError, Node};

/// Check basic DSL structure
pub fn check_basic_structure(dsl: &DifyDsl) -> Vec<LintError> {
    let mut errors = Vec::new();

    // Check app section
    if dsl.app.is_none() {
        errors.push(LintError::error("", "root", "Missing 'app' section"));
    }

    // Check workflow section
    let workflow = match &dsl.workflow {
        Some(w) => w,
        None => {
            errors.push(LintError::error("", "root", "Missing 'workflow' section"));
            return errors;
        }
    };

    // Check graph section
    let graph = match &workflow.graph {
        Some(g) => g,
        None => {
            errors.push(LintError::error("", "workflow", "Missing 'graph' section"));
            return errors;
        }
    };

    // Check nodes exist
    let nodes = match &graph.nodes {
        Some(n) if !n.is_empty() => n,
        _ => {
            errors.push(LintError::error("", "graph", "No nodes defined"));
            return errors;
        }
    };

    // Check for start node
    let start_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|n| {
            n.data
                .as_ref()
                .and_then(|d| d.node_type.as_ref())
                .map(|t| t == "start")
                .unwrap_or(false)
        })
        .collect();

    match start_nodes.len() {
        0 => errors.push(LintError::error("", "graph", "Missing start node")),
        1 => {}
        _ => errors.push(LintError::error("", "graph", "Multiple start nodes found")),
    }

    errors
}
