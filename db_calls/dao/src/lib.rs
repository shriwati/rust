use anyhow::{Context, Result};
use clickhouse::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickHouseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    // pub password: String,
}

impl Default for ClickHouseConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8123,
            database: "uk_property".to_string(),
            username: "default".to_string(),
            // password: "".to_string(),
        }
    }
}

impl ClickHouseConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    pub fn with_username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    // pub fn with_password(mut self, password: &str) -> Self {
    //     self.password = password.to_string();
    //     self
    // }

    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(anyhow::anyhow!("Host cannot be empty"));
        }
        if self.database.is_empty() {
            return Err(anyhow::anyhow!("Database cannot be empty"));
        }
        if self.username.is_empty() {
            return Err(anyhow::anyhow!("Username cannot be empty"));
        }
        if self.port == 0 {
            return Err(anyhow::anyhow!("Port must be greater than 0"));
        }
        Ok(())
    }
}

pub struct ClickHouseConnection {
    client: Client,
    config: ClickHouseConfig,
}

impl std::fmt::Debug for ClickHouseConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClickHouseConnection")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl ClickHouseConnection {
    pub fn new(config: ClickHouseConfig) -> Result<Self> {
        config.validate()?;

        let url = format!("http://{}:{}", config.host, config.port);

        let client = Client::default()
            .with_url(url)
            .with_database(&config.database)
            .with_user(&config.username);
            // .with_password(&config.password);

        Ok(Self { client, config })
    }

    pub fn new_default() -> Result<Self> {
        Self::new(ClickHouseConfig::default())
    }

    pub fn new_with_params(host: &str, database: &str) -> Result<Self> {
        let config = ClickHouseConfig::new()
            .with_host(host)
            .with_database(database);
            // .with_password(password);
        Self::new(config)
    }

    pub fn new_with_all_params(
        host: &str,
        port: u16,
        database: &str,
        username: &str,
        // password: &str,
    ) -> Result<Self> {
        let config = ClickHouseConfig::new()
            .with_host(host)
            .with_port(port)
            .with_database(database)
            .with_username(username);
            // .with_password(password);
        Self::new(config)
    }

    pub async fn test_connection(&self) -> Result<bool> {
        let result = self
            .client
            .query("SELECT 1")
            .fetch_one::<u8>()
            .await
            .context("Failed to test connection to ClickHouse")?;

        Ok(result == 1)
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_config(&self) -> &ClickHouseConfig {
        &self.config
    }
}

pub async fn create_connection() -> Result<ClickHouseConnection> {
    ClickHouseConnection::new_default()
}

pub async fn create_connection_with_config(config: ClickHouseConfig) -> Result<ClickHouseConnection> {
    ClickHouseConnection::new(config)
}

pub async fn create_connection_with_params(
    host: &str,
    database: &str
) -> Result<ClickHouseConnection> {
    ClickHouseConnection::new_with_params(host, database)
}

pub async fn create_connection_with_all_params(
    host: &str,
    port: u16,
    database: &str,
    username: &str
) -> Result<ClickHouseConnection> {
    ClickHouseConnection::new_with_all_params(host, port, database, username)
}

#[cfg(test)]
mod dao_tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config() {
        let config = ClickHouseConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8123);
        assert_eq!(config.database, "uk_property");
        assert_eq!(config.username, "default");
    }

    #[tokio::test]
    async fn test_connection_creation() {
        let connection = create_connection().await;
        assert!(connection.is_ok());
    }

    // #[tokio::test]
    // async fn test_builder_pattern() {
    //     let config = ClickHouseConfig::new()
    //         .with_host("remote.example.com")
    //         .with_port(9000)
    //         .with_database("test_db")
    //         .with_username("test_user");
    //         // .with_password("test_pass");
    //
    //     assert_eq!(config.host, "remote.example.com");
    //     assert_eq!(config.port, 8123);
    //     assert_eq!(config.database, "test_db");
    //     assert_eq!(config.username, "test_user");
    //     assert_eq!(config.password, "test_pass");
    // }

    #[tokio::test]
    async fn test_connection_with_params() {
        let connection = create_connection_with_params(
            "localhost",
            "uk_property",
          
        ).await;
        assert!(connection.is_ok());

        let conn = connection.unwrap();
        assert_eq!(conn.get_config().host, "localhost");
        assert_eq!(conn.get_config().database, "uk_property");
        // assert_eq!(conn.get_config().password, "test_password");
        assert_eq!(conn.get_config().username, "default");
        assert_eq!(conn.get_config().port, 8123);
    }

    #[tokio::test]
    async fn test_connection_with_all_params() {
        let connection = create_connection_with_all_params(
            "localhost",
            9000,
            "uk_property",
            "admin",
            
        ).await;
        assert!(connection.is_ok());

        let conn = connection.unwrap();
        assert_eq!(conn.get_config().host, "localhost");
        assert_eq!(conn.get_config().port, 9000);
        assert_eq!(conn.get_config().database, "uk_property");
        assert_eq!(conn.get_config().username, "admin");
        // assert_eq!(conn.get_config().password, "secure_password");
    }

    #[tokio::test]
    async fn test_config_validation() {
        let invalid_config = ClickHouseConfig::new()
            .with_host("")
            .with_database("test_db");

        let result = ClickHouseConnection::new(invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Host cannot be empty"));
    }

    #[tokio::test]
    async fn test_config_validation_empty_database() {
        let invalid_config = ClickHouseConfig::new()
            .with_host("localhost")
            .with_database("");

        let result = ClickHouseConnection::new(invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database cannot be empty"));
    }

    #[tokio::test]
    async fn test_config_validation_zero_port() {
        let invalid_config = ClickHouseConfig::new()
            .with_host("localhost")
            .with_port(0)
            .with_database("test_db");

        let result = ClickHouseConnection::new(invalid_config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Port must be greater than 0"));
    }
}