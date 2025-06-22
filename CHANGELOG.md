# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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