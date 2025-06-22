use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult};
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    tool_box,
};
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Human: Send + Sync + 'static {
    async fn ask(&self, question: &str) -> anyhow::Result<String>;
}

#[mcp_tool(
    name = "ask_human",
    description = "Ask a human for information that only they would know, such as personal preferences, project-specific context, local environment details, or non-public information",
    idempotent_hint = false,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = false
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct AskHumanTool {
    /// The question to ask the human. Be specific and provide context to help the human understand what information you need.
    question: String,
}
impl AskHumanTool {
    pub async fn call_tool(&self, human: &dyn Human) -> Result<CallToolResult, CallToolError> {
        let answer = human
            .ask(&self.question)
            .await
            .map_err(|e| CallToolError(e.into_boxed_dyn_error()))?;
        Ok(CallToolResult::text_content(answer, None))
    }
}

#[mcp_tool(
    name = "notify_human",
    description = "Report current activity or status to the user without expecting a response",
    idempotent_hint = true,
    destructive_hint = false,
    open_world_hint = false,
    read_only_hint = true
)]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct NotifyHumanTool {
    /// The status or activity to report to the user
    message: String,
}
impl NotifyHumanTool {
    pub async fn call_tool(&self, human: &dyn Human) -> Result<CallToolResult, CallToolError> {
        human
            .ask(&self.message)
            .await
            .map_err(|e| CallToolError(e.into_boxed_dyn_error()))?;
        Ok(CallToolResult::text_content("Status reported successfully".to_string(), None))
    }
}

tool_box!(HumanTools, [AskHumanTool, NotifyHumanTool]);
