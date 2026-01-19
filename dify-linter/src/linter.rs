use crate::checks::{
    check_basic_structure, check_conversation_variables, check_edges, check_model_config,
    check_nodes, check_variable_references,
};
use crate::types::{DifyDsl, LintContext, LintError, LintResult, Severity};

/// Dify DSL Linter
pub struct DifyLinter {
    dsl: DifyDsl,
}

impl DifyLinter {
    /// Create a new linter instance
    pub fn new(dsl: DifyDsl) -> Self {
        Self { dsl }
    }

    /// Run all lint checks and return the result
    pub fn lint(self) -> LintResult {
        let mut all_errors: Vec<LintError> = Vec::new();

        // Check basic structure
        all_errors.extend(check_basic_structure(&self.dsl));

        // Check for critical errors early
        let has_critical = all_errors.iter().any(|e| {
            e.message.contains("Missing 'workflow'")
                || e.message.contains("Missing 'graph'")
                || e.message.contains("No nodes defined")
        });

        if has_critical {
            return split_errors(all_errors);
        }

        // Get workflow components
        let workflow = self.dsl.workflow.as_ref().unwrap();
        let graph = workflow.graph.as_ref().unwrap();
        let nodes: Vec<_> = graph
            .nodes
            .as_ref()
            .map(|n| n.clone())
            .unwrap_or_default();
        let edges: Vec<_> = graph
            .edges
            .as_ref()
            .map(|e| e.clone())
            .unwrap_or_default();
        let conv_vars: Vec<_> = workflow
            .conversation_variables
            .as_ref()
            .map(|c| c.clone())
            .unwrap_or_default();

        // Build context
        let ctx = LintContext::new(&nodes, &edges);

        // Check conversation variables
        all_errors.extend(check_conversation_variables(&conv_vars));

        // Check nodes
        all_errors.extend(check_nodes(&nodes, &ctx));

        // Check edges
        all_errors.extend(check_edges(&edges, &ctx));

        // Check variable references
        all_errors.extend(check_variable_references(&nodes, &ctx));

        // Check model config
        all_errors.extend(check_model_config(&nodes));

        split_errors(all_errors)
    }
}

fn split_errors(all_errors: Vec<LintError>) -> LintResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for err in all_errors {
        match err.severity {
            Severity::Error => errors.push(err),
            Severity::Warning => warnings.push(err),
        }
    }

    LintResult {
        valid: errors.is_empty(),
        errors,
        warnings,
    }
}
