use std::{env, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Env {
    Dev,
    Prod,
    Test,
}

impl FromStr for Env {
    type Err = ();

    fn from_str(input: &str) -> Result<Env, Self::Err> {
        match input {
            "dev" => Ok(Env::Dev),
            "prod" => Ok(Env::Prod),
            "test" => Ok(Env::Test),
            _ => Err(()),
        }
    }
}

impl ToString for Env {
    fn to_string(&self) -> String {
        match self {
            Env::Dev => "dev".to_string(),
            Env::Prod => "prod".to_string(),
            Env::Test => "test".to_string(),
        }
    }
}

pub struct Config {
    pub secret: String,
    pub env: Env,
    pub db_url: String,
}

impl Config {
    pub fn build() -> Config {
        let secret = env::var("SECRET").unwrap_or("secret".to_string());
        let env = env::var("ENV").unwrap_or("dev".to_string());
        let env = Env::from_str(&env).unwrap_or(Env::Dev);
        let db_url = env::var("DB_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

        Config {
            secret,
            env,
            db_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::config::{Config, Env};
    use std::{env, str::FromStr};
    #[tokio::test]
    async fn test_env_to_string() {
        let mut env = Env::Dev;
        assert_eq!(env.to_string(), "dev");

        env = Env::Prod;
        assert_eq!(env.to_string(), "prod");

        env = Env::Test;
        assert_eq!(env.to_string(), "test");
    }
    #[tokio::test]
    async fn test_env_from_string(){
        let mut env = Env::from_str("dev");
        assert_eq!(config.env, Env::Dev);
        env = Env::from_str("prod");
        assert_eq!(config.env, Env::Prod);
        env = Env::from_str("test");
        assert_eq!(config.env, Env::Test);
    }
    #[tokio::test]
    async fn test_build_env() {
        env::set_var("SECRET", "test_secret");
        env::set_var("ENV", "dev");
        env::set_var("DB_URL", "test_db_url");
        let config = Config::build();
        
        assert_eq!(config.secret.clone(), "test_secret");
        assert_eq!(config.env, Env::Dev);
        assert_eq!(config.db_url.clone(), "test_db_url");
    }
}
