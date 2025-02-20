use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[clap(short, long, default_value_t = 16)]
    pub length: u8,

    #[clap(long, default_value_t = true)]
    pub uppercase: bool,

    #[clap(long, default_value_t = true)]
    pub lowercase: bool,

    #[clap(long, default_value_t = true)]
    pub number: bool,

    #[clap(long, default_value_t = true)]
    pub symbol: bool,
}
