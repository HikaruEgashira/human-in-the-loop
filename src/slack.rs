use std::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::{OnceCell, RwLock, broadcast};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use crate::tools::Human;

pub struct HumanInSlack {
    client: Client,
    bot_token: String,
    app_token: String,
    channel_id: String,
    user_id: String,
    thread_ts: OnceCell<String>,
    timeout_minutes: Option<u64>,
    pending_responses: Arc<RwLock<HashMap<String, broadcast::Sender<String>>>>,
    websocket_started: OnceCell<()>,
}

impl HumanInSlack {
    pub fn new(
        bot_token: String,
        app_token: String,
        channel_id: String,
        user_id: String,
        timeout_minutes: Option<u64>,
    ) -> anyhow::Result<Self> {
        let client = Client::new();
        Ok(Self {
            client,
            bot_token,
            app_token,
            channel_id,
            user_id,
            thread_ts: OnceCell::new(),
            timeout_minutes,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            websocket_started: OnceCell::new(),
        })
    }

    pub async fn start_socket_connection(&self) -> anyhow::Result<()> {
        // Get WebSocket URL
        let response = self.client
            .post("https://slack.com/api/apps.connections.open")
            .header("Authorization", format!("Bearer {}", self.app_token))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        let data: Value = response.json().await?;
        
        if !data["ok"].as_bool().unwrap_or(false) {
            return Err(anyhow::anyhow!("Failed to get WebSocket URL: {}", data["error"].as_str().unwrap_or("unknown")));
        }

        let ws_url = data["url"].as_str()
            .ok_or_else(|| anyhow::anyhow!("No WebSocket URL in response"))?;

        // Connect to WebSocket
        let (ws_stream, _) = connect_async(ws_url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        let pending_responses = self.pending_responses.clone();
        let channel_id = self.channel_id.clone();
        let user_id = self.user_id.clone();

        // Handle incoming messages
        tokio::spawn(async move {
            while let Some(message) = ws_receiver.next().await {
                if let Ok(Message::Text(text)) = message {
                    if let Ok(event) = serde_json::from_str::<Value>(&text) {
                        // Acknowledge events
                        if let Some(envelope_id) = event["envelope_id"].as_str() {
                            let ack = json!({
                                "envelope_id": envelope_id
                            });
                            let _ = ws_sender.send(Message::Text(ack.to_string())).await;
                        }

                        // Handle message events
                        if event["type"] == "events_api" {
                            if let Some(event_data) = event["payload"]["event"].as_object() {
                                if event_data["type"] == "message" 
                                    && event_data["channel"] == channel_id
                                    && event_data["user"] == user_id {
                                    
                                    if let Some(text) = event_data["text"].as_str() {
                                        // For thread replies, use thread_ts
                                        if let Some(thread_ts) = event_data["thread_ts"].as_str() {
                                            let responses = pending_responses.read().await;
                                            if let Some(sender) = responses.get(thread_ts) {
                                                let _ = sender.send(text.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

}

#[async_trait]
impl Human for HumanInSlack {
    async fn ask(&self, question: &str) -> anyhow::Result<String> {
        // Ensure socket connection is established (only once)
        self.websocket_started.get_or_try_init(|| async {
            self.start_socket_connection().await?;
            Ok::<(), anyhow::Error>(())
        }).await?;

        // Check if thread already exists
        let is_initial_message = self.thread_ts.get().is_none();
        
        // Get or create thread
        let thread_ts = self.thread_ts.get_or_try_init(|| async {
            let question_title = question.chars().take(100).collect::<String>();
            let message_text = format!("<@{}> {}", self.user_id, question_title);

            let payload = json!({
                "channel": self.channel_id,
                "text": message_text
            });

            let response = self.client
                .post("https://slack.com/api/chat.postMessage")
                .header("Authorization", format!("Bearer {}", self.bot_token))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await?;

            if response.status().is_success() {
                let data: Value = response.json().await?;
                if let Some(ts) = data["ts"].as_str() {
                    Ok(ts.to_string())
                } else {
                    Err(anyhow::anyhow!("Failed to get timestamp from message"))
                }
            } else {
                Err(anyhow::anyhow!("Failed to post message to Slack"))
            }
        }).await?;

        // Subscribe to the thread BEFORE sending any message
        let timeout_duration = self.timeout_minutes
            .map(|m| Duration::from_secs(m * 60))
            .unwrap_or(Duration::from_secs(300));

        let mut receiver = {
            let mut pending = self.pending_responses.write().await;
            let sender = pending.entry(thread_ts.to_string()).or_insert_with(|| {
                let (sender, _) = broadcast::channel(16);
                sender
            });
            sender.subscribe()
        };

        // If this is not the initial message, post the question as a thread reply
        if !is_initial_message {
            let message_text = format!("<@{}> {}", self.user_id, question);
            
            let payload = json!({
                "channel": self.channel_id,
                "text": message_text,
                "thread_ts": thread_ts
            });

            self.client
                .post("https://slack.com/api/chat.postMessage")
                .header("Authorization", format!("Bearer {}", self.bot_token))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await?;
        }

        // Wait for response with timeout
        let result = tokio::time::timeout(timeout_duration, receiver.recv()).await;

        match result {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(broadcast::error::RecvError::Closed)) => Err(anyhow::anyhow!("Channel closed")),
            Ok(Err(broadcast::error::RecvError::Lagged(_))) => Err(anyhow::anyhow!("Message lagged")),
            Err(_) => Ok("Please think autonomously. If you can adopt methods to delay decision-making, please use those approaches. If decision-making is necessary, please record what decisions you made in ./adr/yyyymmdd-hhmmss so you can proceed autonomously.".to_string()),
        }
    }
}