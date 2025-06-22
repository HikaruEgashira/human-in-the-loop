# Discord Setup Guide

This guide will walk you through setting up the Human-in-the-Loop MCP server with Discord.

## Prerequisites

- A Discord account
- Administrator permissions on a Discord server
- Ability to create Discord applications

## Step 1: Create Discord Bot

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Enter an application name (e.g., "Human-in-the-Loop MCP")
4. Click "Create"

## Step 2: Create Bot User

1. In your application settings, go to "Bot" in the sidebar
2. Click "Add Bot"
3. Click "Yes, do it!" to confirm
4. **Important**: Copy and save the bot token - you'll need it as `DISCORD_TOKEN`
5. Under "Privileged Gateway Intents", you may need to enable:
   - Message Content Intent (if your bot needs to read message content)

## Step 3: Set Bot Permissions

1. Go to "OAuth2" → "URL Generator" in the sidebar
2. Under "Scopes", select:
   - `bot`
3. Under "Bot Permissions", select:
   - Send Messages
   - Create Public Threads
   - Read Message History
   - Use Slash Commands (optional)

## Step 4: Invite Bot to Server

1. Copy the generated URL from the OAuth2 URL Generator
2. Open the URL in your browser
3. Select the Discord server where you want to add the bot
4. Click "Authorize"
5. Complete any captcha if prompted

## Step 5: Get Discord IDs

### Channel ID
1. Enable Developer Mode in Discord:
   - Go to Settings (gear icon)
   - Advanced → Developer Mode (toggle ON)
2. Right-click on the channel where you want the bot to operate
3. Select "Copy ID"
4. This is your `DISCORD_CHANNEL_ID`

### User ID
1. Right-click on your username in Discord
2. Select "Copy ID"
3. This is your `DISCORD_USER_ID`

## Step 6: Configure the MCP Server

### Claude Desktop Configuration

Add the following to your Claude Desktop config (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "human-in-the-loop": {
      "command": "human-in-the-loop",
      "args": [
        "--discord-channel-id", "your-channel-id",
        "--discord-user-id", "your-user-id",
        "--timeout", "5"
      ],
      "env": {
        "DISCORD_TOKEN": "your-discord-bot-token"
      }
    }
  }
}
```

### Claude Code Configuration

For Claude Code, add to your MCP settings:

```json
{
  "human-in-the-loop": {
    "command": "human-in-the-loop",
    "args": [
      "--discord-channel-id", "your-channel-id",
      "--discord-user-id", "your-user-id",
      "--timeout", "5"
    ]
  }
}
```

Set the Discord token as an environment variable before running Claude Code:

```bash
export DISCORD_TOKEN="your-discord-bot-token"
claude
```

## Step 7: Test the Setup

1. Restart Claude Desktop or Claude Code
2. In a conversation, the AI should now be able to use the `ask_human` tool
3. When triggered, you should receive a message in your configured Discord channel
4. Reply to the message to send your response back to the AI

## Configuration Options

### Timeout Settings

The `--timeout` parameter controls how long the AI will wait for human responses (default: 5 minutes). When the timeout expires, the AI receives a message encouraging autonomous decision-making:

- If decision-making can be delayed, the AI should adopt those approaches
- If decisions must be made, the AI should document them in `./adr/yyyymmdd-hhmmss` format
- This allows the AI to proceed autonomously when humans are unavailable

### Alternative Token Passing

You can also pass the Discord token directly as a command line argument:

```json
{
  "mcpServers": {
    "human-in-the-loop": {
      "command": "human-in-the-loop",
      "args": [
        "--discord-token", "your-discord-bot-token",
        "--discord-channel-id", "your-channel-id",
        "--discord-user-id", "your-user-id",
        "--timeout", "5"
      ]
    }
  }
}
```

## How It Works

1. AI assistant calls the `ask_human` tool
2. MCP server creates a thread in the specified Discord channel (or uses existing thread)
3. Posts the question and mentions the specified user
4. Waits for user's reply with configurable timeout
5. Returns the reply content to the AI assistant

## Troubleshooting

### Bot Not Responding
- Verify the bot is added to the correct server and channel
- Check that the bot has the required permissions
- Ensure the Discord token is valid and not expired

### Permission Issues
- Make sure the bot has "Send Messages" permission in the target channel
- Verify "Create Public Threads" permission if using thread functionality
- Check "Read Message History" permission for reading replies

### ID Issues
- Ensure Developer Mode is enabled in Discord to copy IDs
- Verify the channel ID and user ID are correct
- Channel IDs are typically 18-19 digits long

## Security Notes

- Keep your bot token secure and never commit it to version control
- Use environment variables for production deployments
- Regularly rotate your bot token if needed
- The bot only responds to messages from the specified user ID for security