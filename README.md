# Human-in-the-Loop MCP Server

An MCP (Model Context Protocol) server that allows AI assistants to ask questions to humans via Discord.

## Overview

This MCP server enables AI assistants to request human input or judgment during their work. For example:

- When creating documentation, the AI designs the structure while humans provide specific content
- When the AI needs confirmation on uncertain decisions
- When specialized knowledge or personal information is required

## Requirements

- Rust (1.70 or higher)
- Discord account and bot
- MCP-compatible AI client (Claude Desktop, Claude Code, etc.)

## Quick Start

1. **Setup Discord Bot**: Follow the [Discord Setup Guide](docs/discord.md)
2. **Install**: `cargo install --git https://github.com/KOBA789/human-in-the-loop.git`
3. **Configure your MCP client** with the server details (see setup guide)

## Documentation

- [Discord Setup Guide](docs/discord.md) - Complete Discord bot setup and configuration
- [Slack Setup Guide](docs/slack.md) - Complete Slack app setup and configuration

## How It Works

1. AI assistant calls the `ask_human` tool when it needs human input
2. MCP server posts the question in Discord and mentions the specified user
3. Human responds in Discord
4. Response is returned to the AI assistant
5. AI continues with the human-provided information

## Configuration Options

- **Timeout**: Configure how long to wait for human responses (default: 5 minutes)
- **Auto-fallback**: When timeout expires, AI receives guidance for autonomous decision-making
- **Thread support**: Questions are organized in threads for better conversation flow

## Example Usage

```
Human: Please create a documentation outline. You can ask me questions as needed.
Assistant: I'll create a documentation outline. Let me ask you some questions first.
[Uses ask_human tool to gather requirements]
```

## Future Plans

- **Migration to MCP Elicitation**: Once MCP's Elicitation implementation becomes standardized, we plan to migrate from Discord/Slack to native MCP Elicitation for a more integrated experience.
