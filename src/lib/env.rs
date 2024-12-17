use crate::application::ports::messaging_ports::MessagingType;
use clap::Parser;

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub messaging_type: MessagingType,

    #[clap(env)]
    pub database_host: String,
    #[clap(env)]
    pub database_user: String,
    #[clap(env)]
    pub database_password: String,
    #[clap(env)]
    pub database_name: String,

    #[clap(env)]
    pub nats_url: Option<String>,

    #[clap(env)]
    pub google_project_id: Option<String>,
}
