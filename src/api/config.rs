use std::{env, str::FromStr,process};

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
    pub user_pool_id_cliente: String,
    pub user_pool_id_usuario: String,
}

impl Config {
    pub fn build() -> Config {
        let secret = env::var("SECRET").unwrap_or("secret".to_string());
        let env = env::var("ENV").unwrap_or("dev".to_string());
        let env = Env::from_str(&env).unwrap_or(Env::Dev);
        let user_pool_id_cliente = match std::env::var("AWS_COGNITO_USER_POOL_ID_CLIENTE") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("AWS_COGNITO_USER_POOL_ID_CLIENTE environment variable not set.");
                process::exit(1);
            }
        };

        let user_pool_id_usuario = match std::env::var("AWS_COGNITO_USER_POOL_ID_USUARIO") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("AWS_COGNITO_USER_POOL_ID_USUARIO environment variable not set.");
                process::exit(1);
            }
        };

        Config {
            secret,
            env,
            user_pool_id_cliente,
            user_pool_id_usuario
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
        let mut env = Env::from_str("dev").unwrap();
        assert_eq!(env, Env::Dev);
        env = Env::from_str("prod").unwrap();
        assert_eq!(env, Env::Prod);
        env = Env::from_str("test").unwrap();
        assert_eq!(env, Env::Test);
    }
    
    #[tokio::test]
    async fn test_build_env() {
        env::set_var("SECRET", "test_secret");
        env::set_var("ENV", "dev");
        env::set_var("AWS_COGNITO_USER_POOL_ID_CLIENTE", "test_cliente_pool");
        env::set_var("AWS_COGNITO_USER_POOL_ID_USUARIO", "test_cliente_usuario");
        let config = Config::build();
        
        assert_eq!(config.secret.clone(), "test_secret");
        assert_eq!(config.env, Env::Dev);
    }
}
