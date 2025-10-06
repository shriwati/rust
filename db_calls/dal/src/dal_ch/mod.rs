use anyhow::{anyhow, Result};
use clickhouse::Client;

pub struct ClickHouseConnection {
    client: Client,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub _password: String,
}

impl std::fmt::Debug for ClickHouseConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickHouseConnection")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("database", &self.database)
            .field("username", &self.username)
            .finish_non_exhaustive()
    }
}

impl ClickHouseConnection {
    pub fn new(host: &str, port: u16, username: &str, database: &str) -> Result<Self> {
        ClickHouseConnection::validate(host, port, username, database)?;

        let url = format!("http://{}:{}", host, port);

        let client = Client::default()
            .with_url(url)
            .with_database(database)
            .with_user(username);

        Ok(Self {
            client,
            host: host.to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            _password: String::new(),
        })
    }

    fn validate(host: &str, port: u16, _username: &str, database: &str) -> Result<()> {
        if host.is_empty() {
            return Err(anyhow!("Host cannot be empty"));
        }
        if port == 0 {
            return Err(anyhow!("Port must be greater than 0"));
        }
        if database.is_empty() {
            return Err(anyhow!("Database cannot be empty"));
        }
        Ok(())
    }

    pub async fn test_connection(&self) -> Result<bool> {
        let result = self
            .client
            .query("SELECT 1")
            .fetch_one::<u8>()
            .await?;

        Ok(result == 1)
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }
}

pub async fn create_connection(
    host: &str,
    port: u16,
    database: &str,
    username: &str,
) -> Result<ClickHouseConnection> {
    ClickHouseConnection::new(host, port, username, database)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection_creation() {
        let connection = create_connection("127.0.0.1", 8123, "uk_property", "default").await;
        assert!(connection.is_ok());
    }
}
