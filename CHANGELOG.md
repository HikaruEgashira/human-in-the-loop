# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- **TLS Support**: Added native-tls feature to tokio-tungstenite for secure WebSocket connections
- **Code Quality**: Removed unused `message_receiver` field from HumanInSlack struct to eliminate compiler warnings

## [0.3.0] - 2025-01-22

### Added
- **Slack Integration**: Added support for human communication via Slack
  - New `slack` subcommand with `--bot-token`, `--app-token`, `--channel-id`, and `--user-id` options
  - Environment variable support: `SLACK_BOT_TOKEN`, `SLACK_APP_TOKEN`, `SLACK_CHANNEL_ID`, `SLACK_USER_ID`
  - Thread-based conversation support for multiple questions
  - Socket Mode connection for real-time message handling
- **Multi-provider Architecture**: Refactored to support both Discord and Slack
  - Provider-based command structure using clap subcommands
  - Shared timeout configuration across providers

### Changed
- **Breaking**: Command line interface now requires provider selection (`discord` or `slack`)
- Updated help text to mention both Discord and Slack support
- Improved error handling for provider initialization

### Technical Details
- Added `reqwest`, `tokio-tungstenite`, and `futures-util` dependencies for Slack integration
- Implemented WebSocket-based real-time communication with Slack
- Maintained backward compatibility for Discord functionality

## [0.2.0] - 2025-01-22

### Added
- `--timeout` command line argument to set timeout for human responses in minutes
- Default timeout of 5 minutes for human responses
- Autonomous decision-making guidance when timeout expires
- AI receives instructions to delay decisions when possible or document decisions in ADR format

### Changed
- Human response waiting behavior now has configurable timeout instead of indefinite waiting
- Updated configuration examples in README to include timeout parameter

## [0.1.0] - Initial Release

### Added
- Basic Human-in-the-Loop MCP server functionality
- Discord integration for AI-human communication
- `ask_human` tool for AI assistants to request human input
- Support for Claude Desktop and Claude Code integration
- Thread-based conversation management in Discord
- Environment variable configuration support