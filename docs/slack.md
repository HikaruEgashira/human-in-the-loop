# Slack Setup Guide

This guide will walk you through setting up the Human-in-the-Loop MCP server with Slack.

## Prerequisites

- A Slack workspace where you have admin permissions
- Ability to create and manage Slack apps

## Step 1: Create a Slack App

1. Go to [Slack API: Your Apps](https://api.slack.com/apps)
2. Click "Create New App"
3. Choose "From scratch"
4. Enter an app name (e.g., "Human-in-the-Loop MCP")
5. Select your workspace
6. Click "Create App"

## Step 2: Configure Bot Token Scopes

1. In your app settings, go to "OAuth & Permissions" in the sidebar
2. Scroll down to "Scopes" → "Bot Token Scopes"
3. Add the following scopes:
   - `chat:write` - Send messages as the bot
   - `chat:write.public` - Send messages to channels the bot isn't a member of
   - `channels:read` - View basic information about public channels

## Step 3: Enable Socket Mode

1. Go to "Socket Mode" in the sidebar
2. Toggle "Enable Socket Mode" to ON
3. Under "App-Level Tokens", click "Generate Token and Scopes"
4. Enter a token name (e.g., "socket-token")
5. Add the `connections:write` scope
6. Click "Generate"
7. **Important**: Copy and save this App-Level Token - you'll need it as `SLACK_APP_TOKEN`

## Step 4: Subscribe to Events

1. Go to "Event Subscriptions" in the sidebar
2. Toggle "Enable Events" to ON
3. Under "Subscribe to bot events", add:
   - `message.channels` - Listen for messages in public channels

## Step 5: Install the App

1. Go to "Install App" in the sidebar
2. Click "Install to Workspace"
3. Review the permissions and click "Allow"
4. **Important**: Copy and save the "Bot User OAuth Token" (starts with `xoxb-`) - you'll need it as `SLACK_BOT_TOKEN`

## Step 6: Get Channel and User IDs

### Channel ID
1. Open Slack in your browser
2. Navigate to the channel where you want the bot to operate
3. Look at the URL - the channel ID is the part after `/channels/` (e.g., `C1234567890`)
4. Alternatively, right-click the channel name → "Copy link" → extract the ID from the URL

### User ID
1. Click on your profile picture in Slack
2. Select "Profile"
3. Click the "More" button (three dots)
4. Select "Copy member ID"
5. Alternatively, you can find it in your profile URL or by using the Slack API

## Step 7: Configure the MCP Server

Add the following to your Claude Desktop config (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "human-in-the-loop": {
      "command": "human-in-the-loop",
      "args": [
        "slack",
        "--bot-token", "xoxb-your-bot-token",
        "--app-token", "xapp-your-app-token", 
        "--channel-id", "C1234567890",
        "--user-id", "U1234567890",
        "--timeout", "5"
      ]
    }
  }
}
```

Or use environment variables:

```json
{
  "mcpServers": {
    "human-in-the-loop": {
      "command": "human-in-the-loop",
      "args": [
        "slack",
        "--channel-id", "C1234567890",
        "--user-id", "U1234567890",
        "--timeout", "5"
      ],
      "env": {
        "SLACK_BOT_TOKEN": "xoxb-your-bot-token",
        "SLACK_APP_TOKEN": "xapp-your-app-token"
      }
    }
  }
}
```

## Step 8: Test the Setup

1. Restart Claude Desktop
2. In a conversation, the AI should now be able to use the `ask_human` tool
3. When triggered, you should receive a message in your configured Slack channel
4. Reply to the thread to send your response back to the AI

## Troubleshooting

### "TLS support not compiled in" Error
Make sure you're using version 0.3.1 or later, which includes the TLS fix.

### Bot Not Responding
- Verify the bot is added to the channel
- Check that all token scopes are correctly configured
- Ensure Socket Mode is enabled with the correct app-level token

### Message Not Received
- Confirm the channel ID and user ID are correct
- Check that the bot has permission to post in the channel
- Verify the Event Subscriptions are properly configured

## Security Notes

- Keep your bot tokens secure and never commit them to version control
- Use environment variables for production deployments
- Regularly rotate your tokens if needed
- The bot only responds to messages from the specified user ID for security