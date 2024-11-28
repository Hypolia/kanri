use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub nats_url: String,

    #[clap(env)]
    pub database_host: String,
    #[clap(env)]
    pub database_user: String,
    #[clap(env)]
    pub database_password: String,
}
