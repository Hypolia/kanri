use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub database_url: String,
}
