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
  -o, --output <FILE>    Output MP3 file path
  -s, --speed <RATE>     Speech rate 0.2 to 5.0 (default: 1.0)
  -v, --voice <VOICE>    Polly voice name (default: Joanna)
  -m, --model <MODEL_ID> Bedrock model ID (default: us.anthropic.claude-haiku-4-5-20251001-v1:0)
  -h, --help             Print help
```

## Output File

If `--output` is not specified, the file is saved automatically:

```
~/readouter/output_{title}_{yyyyMMddHHmmss}.mp3
```

- `{title}` is derived from the `<title>` tag of the fetched HTML (non-alphanumeric characters are replaced with `_`)
- `{yyyyMMddHHmmss}` is the local datetime at the time of execution
- The output directory is created automatically if it does not exist

### Environment Variables

| Variable | Description |
|----------|-------------|
| `AWS_PROFILE` | AWS SSO profile name |
| `AWS_DEFAULT_REGION` | AWS region (recommended: `us-east-1`) |
| `READOUTER_OUTPUT_DIR` | Override the default output directory (`~/readouter`) |

### Examples

```bash
# Basic usage — saved to ~/readouter/output_{title}_{timestamp}.mp3
readouter https://example.com/article

# Custom speed and voice
readouter https://example.com/article --speed 0.8 --voice Matthew

# Specify output path explicitly
readouter https://example.com/article -o article.mp3

# Change output directory via environment variable
READOUTER_OUTPUT_DIR=/tmp/audio readouter https://example.com/article

# Use a more capable Bedrock model
readouter https://example.com/article --model us.anthropic.claude-sonnet-4-5-20251001-v1:0
```

## Bedrock Model
The default model is **Claude Haiku 4.5** (`us.anthropic.claude-haiku-4-5-20251001-v1:0`).

## Supported Polly Neural Voices
readouter uses Amazon Polly. Here is supported neural voices.
| Voice ID | Language   | Gender |
|----------|------------|--------|
| Joanna   | US English | Female |
| Matthew  | US English | Male   |
| Ivy      | US English | Female |
| Justin   | US English | Male   |
