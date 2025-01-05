use super::LoginSessionError;
use crate::authentication_client::{AuthenticationClient, AuthenticationClientConstructorOptions};
use crate::helpers::DEFAULT_USER_AGENT;
use crate::transports::Transport;
use crate::enums::EAuthTokenPlatformType;

#[derive(Debug)]
pub struct LoginSessionOptions<T> {
    pub transport: T,
    pub client: reqwest::Client,
    pub platform_type: EAuthTokenPlatformType,
    pub user_agent: Option<&'static str>,
    pub machine_id: Option<Vec<u8>>,
}

pub fn create_handler<T>(
    transport: T,
    client: reqwest::Client,
    platform_type: EAuthTokenPlatformType,
    machine_id: Option<Vec<u8>>,
    user_agent: Option<&'static str>,
) -> Result<AuthenticationClient<T>, LoginSessionError>
where
    T: Transport,
{
    Ok(AuthenticationClient::new(AuthenticationClientConstructorOptions {
        platform_type,
        transport,
        client,
        machine_id,
        user_agent: user_agent.unwrap_or(DEFAULT_USER_AGENT),
    }))
}