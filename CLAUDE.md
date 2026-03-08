# readouter — Project Guide for Claude

## Overview

`readouter` is a Rust CLI tool that converts an article URL into an MP3 file.
It fetches raw HTML, extracts the article body via Amazon Bedrock (Claude), and synthesizes speech via Amazon Polly.

## Architecture

```
URL
 ↓
[extractor.rs] HTTP fetch (reqwest blocking) → raw HTML
 ↓
[extractor.rs] Amazon Bedrock (Claude) → plain article text
 ↓
[polly.rs]     Split into ≤2800-char chunks → wrap each chunk in SSML <prosody>
 ↓
[polly.rs]     Amazon Polly Neural TTS → MP3 bytes per chunk
 ↓
[audio.rs]     Concatenate MP3 chunks → write output file
```

## File Structure

| File | Responsibility |
|------|----------------|
| `src/main.rs` | Entry point; orchestrates the full pipeline |
| `src/cli.rs` | CLI argument definitions via `clap` derive |
| `src/extractor.rs` | HTTP fetch + Bedrock invocation for article extraction |
| `src/polly.rs` | Text chunking, SSML generation, Polly synthesis |
| `src/audio.rs` | MP3 chunk concatenation and file output |

## Key Design Decisions

- **AWS auth**: Uses the default provider chain (`aws_config::load_defaults`). No hardcoded credentials. Relies on AWS SSO tokens.
- **Bedrock model**: Configurable via `--model`. Default is `anthropic.claude-3-haiku-20240307-v1:0`.
- **Polly engine**: Always `Neural` for natural-sounding speech.
- **Chunk limit**: 2800 chars per chunk (Polly SSML limit is 3000; 200-char margin for the SSML wrapper).
- **MP3 concatenation**: Binary concatenation of MP3 frames — no re-encoding needed.
- **Speed mapping**: `--speed 0.8` → `<prosody rate="80%">`.

## AWS Permissions Required

- `bedrock:InvokeModel` on the target model ARN
- `polly:SynthesizeSpeech`

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `AWS_PROFILE` | AWS SSO profile name |
| `AWS_DEFAULT_REGION` | AWS region (recommend `us-east-1`) |

## Development Notes

- Run `cargo build` to verify compilation.
- `reqwest` is used in blocking mode inside a `tokio` async context via `tokio::task::spawn_blocking` is NOT used — the blocking call happens before the async runtime does heavy work, which is acceptable for this CLI use case.
- Avoid upgrading dependency versions without checking breaking changes in `aws-sdk-*` crates, as they version-lock together via `aws-config`.
