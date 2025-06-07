# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ECS Navigator (ecs-nav) is a Rust CLI tool that simplifies using AWS ECS exec by providing an interactive interface to select clusters, services, tasks, and containers. It uses the AWS SDK for Rust and the dialoguer crate for interactive prompts.

## Common Commands

### Build Commands
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized release version
- `cargo check` - Check code for errors without building
- `cargo clippy` - Run linter for Rust best practices

### Development Commands
- `cargo run` - Run the application in debug mode
- `cargo test` - Run tests (if any exist)
- `cargo fmt` - Format code according to Rust standards

### Installation
- `cargo install --path .` - Install ecs-nav locally from source

## Architecture

The codebase follows a simple modular structure:

1. **main.rs**: Entry point that orchestrates the workflow:
   - Creates AWS client
   - Lists clusters → services → tasks → containers
   - Executes the selected command using AWS CLI

2. **aws_client.rs**: AWS SDK configuration
   - Creates ECS client with region chain (defaults to us-west-2)

3. **cli.rs**: Interactive UI components
   - Uses dialoguer for terminal-based selection menus

4. **ecs.rs**: Core ECS functionality
   - Data structures: Cluster, Service, Task, Container (all implement FromArn trait)
   - AWS API calls: list_clusters, list_services, list_tasks, list_containers
   - Command execution: Uses execvp to replace process with AWS CLI ecs execute-command

## Key Design Patterns

- **FromArn Trait**: Extracts resource names from AWS ARNs
- **Error Handling**: Converts AWS SDK errors to user-friendly messages
- **Pagination**: Handles paginated results for services listing
- **Process Replacement**: Uses execvp instead of spawning subprocess for better terminal control

## Dependencies

- AWS SDK (aws-config, aws-sdk-ecs)
- tokio for async runtime
- dialoguer for interactive CLI
- nix for Unix process operations