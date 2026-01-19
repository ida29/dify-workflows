use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Lint error severity
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

/// A single lint error or warning
#[derive(Debug, Clone, Serialize)]
pub struct LintError {
    pub severity: Severity,
    pub node_id: String,
    pub node_title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_hint: Option<String>,
}

impl LintError {
    pub fn error(node_id: &str, node_title: &str, message: &str) -> Self {
        Self {
            severity: Severity::Error,
            node_id: node_id.to_string(),
            node_title: node_title.to_string(),
            message: message.to_string(),
            fix_hint: None,
        }
    }

    pub fn error_with_hint(node_id: &str, node_title: &str, message: &str, hint: &str) -> Self {
        Self {
            severity: Severity::Error,
            node_id: node_id.to_string(),
            node_title: node_title.to_string(),
            message: message.to_string(),
            fix_hint: Some(hint.to_string()),
        }
    }

    pub fn warning(node_id: &str, node_title: &str, message: &str) -> Self {
        Self {
            severity: Severity::Warning,
            node_id: node_id.to_string(),
            node_title: node_title.to_string(),
            message: message.to_string(),
            fix_hint: None,
        }
    }
}

/// Root Dify DSL structure
#[derive(Debug, Deserialize)]
pub struct DifyDsl {
    pub app: Option<App>,
    pub workflow: Option<Workflow>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// App section
#[derive(Debug, Deserialize)]
pub struct App {
    pub name: Option<String>,
    pub mode: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Workflow section
#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub conversation_variables: Option<Vec<ConversationVariable>>,
    pub graph: Option<Graph>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Conversation variable
#[derive(Debug, Clone, Deserialize)]
pub struct ConversationVariable {
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<Value>,
    pub value_type: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Graph containing nodes and edges
#[derive(Debug, Deserialize)]
pub struct Graph {
    pub nodes: Option<Vec<Node>>,
    pub edges: Option<Vec<Edge>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// A workflow node
#[derive(Debug, Deserialize)]
pub struct Node {
    pub id: Option<String>,
    pub data: Option<NodeData>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Node data containing type-specific configuration
#[derive(Debug, Deserialize)]
pub struct NodeData {
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    pub title: Option<String>,
    pub model: Option<Model>,
    pub prompt_template: Option<Value>,
    pub conditions: Option<Vec<ConditionGroup>>,
    pub classes: Option<Vec<ClassDefinition>>,
    pub query_variable_selector: Option<Vec<String>>,
    pub variables: Option<Value>,
    pub answer: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Model configuration
#[derive(Debug, Deserialize)]
pub struct Model {
    pub name: Option<String>,
    pub provider: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Condition group for IF/ELSE nodes
#[derive(Debug, Deserialize)]
pub struct ConditionGroup {
    pub id: Option<String>,
    pub conditions: Option<Vec<Condition>>,
    pub logical_operator: Option<String>,
}

/// Single condition
#[derive(Debug, Deserialize)]
pub struct Condition {
    pub variable_selector: Option<Vec<String>>,
    pub comparison_operator: Option<String>,
    pub value: Option<Value>,
}

/// Class definition for Question Classifier
#[derive(Debug, Deserialize)]
pub struct ClassDefinition {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// Edge connecting nodes
#[derive(Debug, Deserialize)]
pub struct Edge {
    pub id: Option<String>,
    pub source: Option<String>,
    pub target: Option<String>,
    #[serde(rename = "sourceHandle")]
    pub source_handle: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Lint result containing all errors and warnings
#[derive(Debug, Serialize)]
pub struct LintResult {
    pub valid: bool,
    pub errors: Vec<LintError>,
    pub warnings: Vec<LintError>,
}

/// Context for linting operations
pub struct LintContext {
    pub node_ids: HashSet<String>,
    pub node_map: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

impl LintContext {
    pub fn new(nodes: &[Node], edges: &[Edge]) -> Self {
        let mut node_ids = HashSet::new();
        let mut node_map = HashMap::new();

        for node in nodes {
            if let Some(id) = &node.id {
                node_ids.insert(id.clone());
                node_map.insert(id.clone(), node.clone());
            }
        }

        Self {
            node_ids,
            node_map,
            edges: edges.to_vec(),
        }
    }

    pub fn node_exists(&self, id: &str) -> bool {
        self.node_ids.contains(id)
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.node_map.get(id)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            data: self.data.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Clone for NodeData {
    fn clone(&self) -> Self {
        Self {
            node_type: self.node_type.clone(),
            title: self.title.clone(),
            model: self.model.clone(),
            prompt_template: self.prompt_template.clone(),
            conditions: self.conditions.clone(),
            classes: self.classes.clone(),
            query_variable_selector: self.query_variable_selector.clone(),
            variables: self.variables.clone(),
            answer: self.answer.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            provider: self.provider.clone(),
            extra: self.extra.clone(),
        }
    }
}

impl Clone for ConditionGroup {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            conditions: self.conditions.clone(),
            logical_operator: self.logical_operator.clone(),
        }
    }
}

impl Clone for Condition {
    fn clone(&self) -> Self {
        Self {
            variable_selector: self.variable_selector.clone(),
            comparison_operator: self.comparison_operator.clone(),
            value: self.value.clone(),
        }
    }
}

impl Clone for ClassDefinition {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
        }
    }
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            source: self.source.clone(),
            target: self.target.clone(),
            source_handle: self.source_handle.clone(),
            extra: self.extra.clone(),
        }
    }
}
