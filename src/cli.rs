use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "readouter")]
#[command(about = "Convert article URL to MP3 using Amazon Polly and Bedrock")]
pub struct Args {
    /// Target article URL
    pub url: String,

    /// Output MP3 file path (default: ~/readouter/output_{title}_{timestamp}.mp3)
    #[arg(short, long)]
    pub output: Option<String>,

    /// Speech rate (0.2 to 5.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub speed: f64,

    /// Polly voice name
    #[arg(short, long, default_value = "Joanna")]
    pub voice: String,

    /// Bedrock model ID
    #[arg(short, long, default_value = "us.anthropic.claude-haiku-4-5-20251001-v1:0")]
    pub model: String,
}
