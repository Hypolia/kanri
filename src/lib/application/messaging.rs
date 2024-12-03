use crate::application::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};
use crate::domain::server::models::server::ServerCreationResponse;
use crate::domain::server::ports::ServerService;
use anyhow::Result;
use std::sync::Arc;

pub async fn start_subscriptions<S>(
    messaging: Arc<MessagingTypeImpl>,
    server_service: Arc<S>,
) -> Result<()>
where
    S: ServerService,
{
    let messaging = Arc::clone(&messaging);

    messaging
        .subscribe("server_creation_responses", {
            let server_service = Arc::clone(&server_service);
            move |message: ServerCreationResponse| {
                let server_service = Arc::clone(&server_service);
                async move {
                    server_service.handle_server_created(message).await?;
                    Ok(())
                }
            }
        })
        .await?;

    Ok(())
}
