
# ECS Navigator (ecs-nav)

ECS Navigator (ecs-nav) is a Rust-based CLI tool designed to simplify the use of ecs-exec in Amazon ECS. This tool allows you to interactively select ECS clusters, services, and tasks to execute commands within containers.

## Features

- List ECS clusters interactively.
- Select an ECS cluster and list services within it.
- Select a service and list tasks within it.
- Execute commands in containers interactively.

## Installation

### Using `cargo install` from GitHub

To install ecs-nav using `cargo install` from GitHub, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

1. Install ecs-nav:
   ```sh
   cargo install --git https://github.com/bonyuta0204/ecs-nav.git
   ```

### From Source

To install ecs-nav from source, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
   ```sh
   git clone https://github.com/bonyuta0204/ecs-nav.git
   cd ecs-nav
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

3. Run the project:
   ```sh
   ./target/release/ecs-nav
   ```

### From Binary

1. Download the binary from the [Release page](https://github.com/bonyuta0204/ecs-nav/releases).

2. Put the binary file in a directory that is included in your `$PATH`.

3. You can now start using `ecs-nav` by running:
   ```sh
   ecs-nav
   ```

## System Requirements

- AWS CLI
- AWS Session Manager installed

## Usage

1. Run the `ecs-nav` command:
   ```sh
   ecs-nav
   ```

2. Follow the prompts to select a cluster, service, and task.

3. Execute the desired command in the container.

### Command Line Options

- `--shell` or `-s`: Specify the shell to use for connecting to containers (default: `/bin/bash`)
  
  Example for Alpine containers:
  ```sh
  ecs-nav --shell /bin/sh
  ```

Note: For containers without bash (like Alpine Linux), use the `--shell /bin/sh` option.

## Configuration

Ensure you have AWS credentials configured on your machine. You can set them up using the AWS CLI:
```sh
aws configure
```

## Developer Notes

- This project uses the AWS SDK for Rust to interact with Amazon ECS.
- The `dialoguer` crate is used for interactive command-line prompts.
- The project structure is simple, with the main logic contained in `src/main.rs` and AWS client configuration in `src/aws_client.rs`.
- Feel free to expand the functionality by adding more features or improving existing ones.
- If you encounter any issues or have suggestions, please open an issue or a pull request on the repository.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests.

## Releasing

This project uses [cargo-release](https://github.com/crate-ci/cargo-release) for version management and releases.

### Prerequisites

1. Install cargo-release:
   ```sh
   cargo install cargo-release
   ```

2. Ensure you have push access to the repository and GitHub releases.

### Release Process

1. Update the CHANGELOG.md with your changes under the `[Unreleased]` section.

2. Run a dry-run to see what will happen:
   ```sh
   cargo release patch --dry-run  # for patch version (0.0.4 -> 0.0.5)
   cargo release minor --dry-run  # for minor version (0.0.4 -> 0.1.0)
   cargo release major --dry-run  # for major version (0.0.4 -> 1.0.0)
   ```

3. Execute the release:
   ```sh
   cargo release patch --execute
   ```

   This will automatically:
   - Update the version in `Cargo.toml`
   - Update `CHANGELOG.md` with the new version and date
   - Create a git commit with message "chore: release X.Y.Z"
   - Create a git tag "vX.Y.Z"
   - Push the commit and tag to GitHub
   - Trigger the GitHub Actions workflow to build and publish binaries

4. The GitHub Actions workflow will automatically:
   - Build binaries for macOS (Intel and Apple Silicon)
   - Create a GitHub release with the binaries attached

### Manual Release (Alternative)

If you prefer to release manually:

1. Update version in `Cargo.toml`
2. Update `Cargo.lock`: `cargo update -p ecs-nav`
3. Update CHANGELOG.md
4. Commit: `git commit -am "chore: release 0.0.5"`
5. Tag: `git tag v0.0.5`
6. Push: `git push && git push --tags`
