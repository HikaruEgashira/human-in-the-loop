use std::sync::{Arc, OnceLock};
use std::time::Duration;

use serenity::{
    all::{
        AutoArchiveDuration, ChannelId, ChannelType, Context, CreateMessage, CreateThread,
        EventHandler, GatewayIntents, Ready, UserId,
    },
    Client,
};
use tokio::sync::OnceCell;

use crate::tools::Human;

pub async fn start(discord_token: &str, handler: Handler) -> anyhow::Result<()> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(handler)
        .await?;
    Ok(client.start().await?)
}

#[derive(Clone)]
pub struct Handler {
    ctx: Arc<OnceLock<Context>>,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            ctx: Arc::new(OnceLock::new()),
        }
    }
}

#[async_trait::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        self.ctx.set(ctx).ok();
    }
}

pub struct HumanInDiscord {
    user_id: UserId,
    channel_id: ChannelId,
    handler: Handler,
    thread: OnceCell<ChannelId>,
    timeout_minutes: Option<u64>,
}

impl HumanInDiscord {
    pub fn new(user_id: UserId, channel_id: ChannelId, timeout_minutes: Option<u64>) -> Self {
        Self {
            user_id,
            channel_id,
            handler: Handler::default(),
            thread: OnceCell::new(),
            timeout_minutes,
        }
    }

    pub fn handler(&self) -> &Handler {
        &self.handler
    }
}

#[async_trait::async_trait]
impl Human for HumanInDiscord {
    async fn ask(&self, question: &str) -> anyhow::Result<String> {
        let ctx = self
            .handler
            .ctx
            .get()
            .ok_or_else(|| anyhow::anyhow!("The connection with Discord is not ready"))?;
        let thread = self
            .thread
            .get_or_try_init(|| async {
                let thread_title = question.chars().take(100).collect::<String>();
                let channel = self
                    .channel_id
                    .create_thread(
                        &ctx.http,
                        CreateThread::new(thread_title)
                            .auto_archive_duration(AutoArchiveDuration::OneDay)
                            .kind(ChannelType::PublicThread),
                    )
                    .await?;
                anyhow::Ok(channel.id)
            })
            .await?;
        let message_text = format!("<@{}> {question}", self.user_id.get());
        thread
            .send_message(&ctx.http, CreateMessage::new().content(message_text))
            .await?;
        
        match self.timeout_minutes {
            Some(timeout_minutes) => {
                let timeout_duration = Duration::from_secs(timeout_minutes * 60);
                match thread.await_reply(ctx).timeout(timeout_duration).await {
                    Some(message) => Ok(message.content),
                    None => Ok("Please think autonomously. If you can adopt methods to delay decision-making, please use those approaches. If decision-making is necessary, please record what decisions you made in ./adr/yyyymmdd-hhmmss so you can proceed autonomously.".to_string()),
                }
            }
            None => {
                let message = thread
                    .await_reply(ctx)
                    .await
                    .ok_or_else(|| anyhow::anyhow!("Failed to await message from the human in Discord"))?;
                Ok(message.content)
            }
        }
    }
}
