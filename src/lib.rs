use serde::{Deserialize, Serialize};

/// A single step in an AI workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum WorkflowStep {
    Install(String),
    GrantPermission(String, String),
    Shell(String),
    Screenshot(String),
    Wait(u64),
}

/// A complete AI workflow definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<WorkflowStep>,
}

/// Parse a YAML string into a `Workflow`.
///
/// # Errors
///
/// Returns an error if the YAML is invalid or does not match the expected schema.
pub fn parse_workflow(yaml: &str) -> Result<Workflow, serde_yaml_ng::Error> {
    serde_yaml_ng::from_str(yaml)
}

/// Validate a workflow, returning a list of error strings.
#[must_use]
pub fn validate_workflow(workflow: &Workflow) -> Vec<String> {
    let mut errors = Vec::new();

    if workflow.name.is_empty() {
        errors.push("workflow name must not be empty".to_string());
    }

    if workflow.steps.is_empty() {
        errors.push("workflow must contain at least one step".to_string());
    }

    for (i, step) in workflow.steps.iter().enumerate() {
        match step {
            WorkflowStep::Install(pkg) => {
                if pkg.is_empty() {
                    errors.push(format!("step {}: Install package name must not be empty", i + 1));
                }
            }
            WorkflowStep::Shell(cmd) => {
                if cmd.is_empty() {
                    errors.push(format!("step {}: Shell command must not be empty", i + 1));
                }
            }
            WorkflowStep::Screenshot(path) => {
                if path.is_empty() {
                    errors.push(format!(
                        "step {}: Screenshot output path must not be empty",
                        i + 1
                    ));
                }
            }
            WorkflowStep::GrantPermission(pkg, perm) => {
                if pkg.is_empty() || perm.is_empty() {
                    errors.push(format!(
                        "step {}: GrantPermission requires both package and permission",
                        i + 1
                    ));
                }
            }
            WorkflowStep::Wait(ms) => {
                if *ms == 0 {
                    errors.push(format!("step {}: Wait duration must be > 0", i + 1));
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_workflow() {
        let yaml = r#"
name: test-workflow
steps:
  - type: Install
    value: com.example.app
  - type: Shell
    value: "echo hello"
  - type: Wait
    value: 1000
"#;
        let wf = parse_workflow(yaml).expect("should parse");
        assert_eq!(wf.name, "test-workflow");
        assert_eq!(wf.steps.len(), 3);
    }

    #[test]
    fn empty_steps_produces_error() {
        let wf = Workflow {
            name: "empty".to_string(),
            steps: vec![],
        };
        let errors = validate_workflow(&wf);
        assert!(errors.iter().any(|e| e.contains("at least one")));
    }

    #[test]
    fn install_step_validation() {
        let wf = Workflow {
            name: "test".to_string(),
            steps: vec![WorkflowStep::Install("com.example.app".to_string())],
        };
        let errors = validate_workflow(&wf);
        assert!(errors.is_empty());
    }

    #[test]
    fn shell_step_empty_command_error() {
        let wf = Workflow {
            name: "test".to_string(),
            steps: vec![WorkflowStep::Shell(String::new())],
        };
        let errors = validate_workflow(&wf);
        assert!(errors.iter().any(|e| e.contains("Shell command")));
    }

    #[test]
    fn wait_step_zero_duration_error() {
        let wf = Workflow {
            name: "test".to_string(),
            steps: vec![WorkflowStep::Wait(0)],
        };
        let errors = validate_workflow(&wf);
        assert!(errors.iter().any(|e| e.contains("Wait duration")));
    }
}
