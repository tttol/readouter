# readouter

A CLI tool that converts an article URL into an MP3 audio file. It extracts the article body using Amazon Bedrock (Claude) and generates speech using Amazon Polly.

## Prerequisites

- Rust 1.75+
- AWS IAM Identity Center (SSO) with permissions for:
  - Amazon Bedrock (`InvokeModel`)
  - Amazon Polly (`SynthesizeSpeech`)

## AWS SSO Setup

```bash
# First-time setup
aws configure sso

# Login before each use
aws sso login --profile <profile-name>

# Set profile and region via environment variables
export AWS_PROFILE=<profile-name>
export AWS_DEFAULT_REGION=us-east-1
```

## Build

```bash
cargo build --release
```

## Usage

```
USAGE:
  readouter <URL> [OPTIONS]

ARGS:
  <URL>  Target article URL

OPTIONS:
  -o, --output <FILE>    Output MP3 file path (default: ./output.mp3)
  -s, --speed <RATE>     Speech rate 0.2 to 5.0 (default: 1.0)
  -v, --voice <VOICE>    Polly voice name (default: Joanna)
  -m, --model <MODEL_ID> Bedrock model ID (default: anthropic.claude-3-haiku-20240307-v1:0)
  -h, --help             Print help
```

### Examples

```bash
# Basic usage
readouter https://example.com/article

# Custom speed and voice
readouter https://example.com/article --speed 0.8 --voice Matthew -o article.mp3

# Use a more capable Bedrock model
readouter https://example.com/article --model anthropic.claude-3-5-sonnet-20241022-v2:0
```

## Supported Polly Neural Voices

| Voice ID | Language    | Gender |
|----------|-------------|--------|
| Joanna   | US English  | Female |
| Matthew  | US English  | Male   |
| Ivy      | US English  | Female |
| Justin   | US English  | Male   |
