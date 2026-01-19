use crate::types::{Edge, LintContext, LintError};

/// Check edge connections
pub fn check_edges(edges: &[Edge], ctx: &LintContext) -> Vec<LintError> {
    let mut errors = Vec::new();

    for edge in edges {
        let edge_id = edge.id.as_deref().unwrap_or("");

        if let Some(source) = &edge.source {
            if !ctx.node_exists(source) {
                errors.push(LintError::error(
                    edge_id,
                    "edge",
                    &format!("Edge source '{}' not found", source),
                ));
            }
        }

        if let Some(target) = &edge.target {
            if !ctx.node_exists(target) {
                errors.push(LintError::error(
                    edge_id,
                    "edge",
                    &format!("Edge target '{}' not found", target),
                ));
            }
        }
    }

    errors
}
