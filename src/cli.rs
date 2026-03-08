use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "readouter")]
#[command(about = "Convert article URL to MP3 using Amazon Polly and Bedrock")]
pub struct Args {
    /// Target article URL
    pub url: String,

    /// Output MP3 file path
    #[arg(short, long, default_value = "./output.mp3")]
    pub output: String,

    /// Speech rate (0.2 to 5.0)
    #[arg(short, long, default_value_t = 1.0)]
    pub speed: f64,

    /// Polly voice name
    #[arg(short, long, default_value = "Joanna")]
    pub voice: String,

    /// Bedrock model ID
    #[arg(short, long, default_value = "anthropic.claude-3-haiku-20240307-v1:0")]
    pub model: String,
}
