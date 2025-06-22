mod discord;
mod mcp_handler;
mod slack;
mod tools;

use clap::Parser;
use discord::HumanInDiscord;
use slack::HumanInSlack;
use rust_mcp_sdk::error::{McpSdkError, SdkResult};
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{
    mcp_server::{server_runtime, ServerRuntime},
    McpServer, StdioTransport, TransportOptions,
};
use serenity::all::{ChannelId, UserId};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    provider: ProviderArgs,
    #[clap(long, help = "Timeout in minutes for human responses", default_value = "5")]
    timeout: u64,
}

#[derive(Debug, Parser)]
enum ProviderArgs {
    Discord {
        #[clap(long, env = "DISCORD_TOKEN")]
        token: String,
        #[clap(long, env = "DISCORD_CHANNEL_ID")]
        channel_id: ChannelId,
        #[clap(long, env = "DISCORD_USER_ID")]
        user_id: UserId,
    },
    Slack {
        #[clap(long, env = "SLACK_BOT_TOKEN")]
        bot_token: String,
        #[clap(long, env = "SLACK_APP_TOKEN")]
        app_token: String,
        #[clap(long, env = "SLACK_CHANNEL_ID")]
        channel_id: String,
        #[clap(long, env = "SLACK_USER_ID")]
        user_id: String,
    },
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    let Args { provider, timeout } = Args::parse();

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Human in the loop".to_string(),
            version: "0.1.0".to_string(),
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some(
            "This is a Human-in-the-Loop MCP server that enables AI assistants to request \
             information from humans via Discord or Slack. Use the 'ask_human' tool when you need \
             information that only a human would know, such as: personal preferences, \
             project-specific context, local environment details, or any information that \
             is not publicly available or documented. The human will be notified and \
             their response will be returned to you."
                .to_string(),
        ),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let transport = StdioTransport::new(TransportOptions::default())?;

    match provider {
        ProviderArgs::Discord { token, channel_id, user_id } => {
            let human = HumanInDiscord::new(user_id, channel_id, Some(timeout));
            let discord = discord::start(&token, human.handler().clone());

            let server: ServerRuntime =
                server_runtime::create_server(server_details, transport, mcp_handler::Handler::new(human));
            let mcp = server.start();

            tokio::select! {
                res = mcp => res?,
                res = discord => res.map_err(|e| McpSdkError::AnyError(e.into_boxed_dyn_error()))?,
            }
        }
        ProviderArgs::Slack { bot_token, app_token, channel_id, user_id } => {
            let human = HumanInSlack::new(bot_token, app_token, channel_id, user_id, Some(timeout))
                .map_err(|e| McpSdkError::AnyError(e.into_boxed_dyn_error()))?;

            // Start socket connection
            human.start_socket_connection().await
                .map_err(|e| McpSdkError::AnyError(e.into_boxed_dyn_error()))?;

            let server: ServerRuntime =
                server_runtime::create_server(server_details, transport, mcp_handler::Handler::new(human));
            
            server.start().await?;
        }
    }
    
    Ok(())
}
