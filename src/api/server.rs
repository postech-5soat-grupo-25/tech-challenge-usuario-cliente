use rocket::response::Redirect;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::*;
use std::collections::HashMap;
use std::process;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::error_handling::generic_catchers;
use super::routes::{auth_route, cliente_route, usuario_route};
use crate::adapters::jwt_authentication_adapter::JWTAuthenticationAdapter;
use crate::api::config::{Config, Env};
use crate::external::postgres;
use crate::gateways::aws_cognito_gateway::AwsCognitoRepository;
use crate::gateways::{
    in_memory_cliente_gateway::InMemoryClienteRepository,
    in_memory_usuario_gateway::InMemoryUsuarioRepository,
    postgres_cliente_gateway::PostgresClienteRepository,
    postgres_usuario_gateway::PostgresUsuarioGateway,
};
use crate::traits::authentication_adapter::AuthenticationAdapter;
use crate::traits::{
    cliente_gateway::ClienteGateway,
    usuario_gateway::UsuarioGateway,
};

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();

    let jwt_authentication_adapter: Arc<dyn AuthenticationAdapter + Sync + Send> =
        Arc::new(JWTAuthenticationAdapter::new(config.secret.clone()));

    println!("Loading environment variables...");
    let usuario_repository: Arc<Mutex<dyn UsuarioGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryUsuarioRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url.clone());
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
            .await
            .unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(
            PostgresUsuarioGateway::new(postgres_connection_manager.client, tables).await,
        ))
    };

    let cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryClienteRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url);

        let user_pool_id = match std::env::var("AWS_COGNITO_USER_POOL_ID") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("AWS_COGNITO_USER_POOL_ID environment variable not set.");
                process::exit(1);
            }
        };
        Arc::new(Mutex::new(
            AwsCognitoRepository::new(String::from(user_pool_id)).await,
        ))
    };

    //let pagamento_adapter: Arc<Mutex<dyn PagamentoAdapter + Sync + Send>> = Arc::new(Mutex::new(MockPagamentoSuccesso {}));

    // Cloning cliente_repository to share ownership
    let cloned_cliente_repository = Arc::clone(&cliente_repository);

    let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
        .await
        .unwrap();
    let tables = postgres::get_tables();


    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
        .merge(("port", 3000));

    rocket::build()
        .mount("/", routes![redirect_to_docs])
        .register("/", generic_catchers())
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                urls: vec![
                    UrlObject::new("Auth", "/auth/openapi.json"),
                    UrlObject::new("Usuarios", "/usuarios/openapi.json"),
                    UrlObject::new("Clientes", "/clientes/openapi.json"),
                ],
                ..Default::default()
            }),
        )
        .mount("/auth", auth_route::routes())
        .mount("/usuarios", usuario_route::routes())
        .mount("/clientes", cliente_route::routes())
        .register("/usuarios", usuario_route::catchers())
        .register("/clientes", cliente_route::catchers())
        .manage(jwt_authentication_adapter)
        .manage(usuario_repository)
        .manage(cliente_repository)
        .configure(server_config)
        .launch()
        .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}
