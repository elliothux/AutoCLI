use opencli_rs_core::{CliCommand, CliError};
use serde_json::Value;
use std::collections::HashMap;

/// Execute a CLI command. This is the main entry point for command execution.
/// At this stage, this is a stub -- pipeline and browser support will be added later.
pub async fn execute_command(
    cmd: &CliCommand,
    _kwargs: HashMap<String, Value>,
) -> Result<Value, CliError> {
    tracing::info!(site = %cmd.site, name = %cmd.name, "Executing command");

    // TODO: Phase 2 -- execute pipeline
    // TODO: Phase 4 -- browser session management

    Err(CliError::command_execution(format!(
        "Command execution not yet implemented: {}",
        cmd.full_name()
    )))
}
