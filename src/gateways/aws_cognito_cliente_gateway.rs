use aws_config::from_env;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::operation::list_users;
use aws_sdk_cognitoidentityprovider::types::AttributeType;
use aws_sdk_cognitoidentityprovider::{config::Region, meta::PKG_VERSION, Client};
use chrono::Utc;
use mockall::{mock, predicate::*};

use crate::{
    base::domain_error::DomainError, entities::cliente::Cliente, entities::cpf::Cpf,
    traits::cliente_gateway::ClienteGateway,
};

fn option_to_string(option: Option<&str>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

pub struct AwsCognitoClienteRepository {
    client: Client,
    user_pool_id: String,
}

impl AwsCognitoClienteRepository {
    pub async fn new(user_pool_id: String) -> Self {
        let region_provider = RegionProviderChain::default_provider();

        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);
        AwsCognitoClienteRepository {
            client,
            user_pool_id,
        }
    }
}

#[async_trait]
impl ClienteGateway for AwsCognitoClienteRepository {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        let response = self
            .client
            .list_users()
            .user_pool_id(&self.user_pool_id)
            .send()
            .await;

        let mut clientes: Vec<Cliente> = Vec::new();

        match response {
            Ok(response) => {
                let users = response.users();

                for user in users {
                    let mut id = String::new();
                    let mut nome = String::new();
                    let mut email = String::new();
                    let mut cpf_string = String::new();
                    let mut data_criacao = String::new();
                    let mut data_atualizacao = String::new();

                    for attr in user.attributes() {
                        match attr.name() {
                            "custom:id" => id =option_to_string(attr.value()),
                            "custom:nome" => nome = option_to_string(attr.value()),
                            "custom:email" => email = option_to_string(attr.value()),
                            "custom:cpf" => cpf_string = option_to_string(attr.value()),
                            "custom:data_criacao" => data_criacao = option_to_string(attr.value()),
                            "custom:data_atualizacao" => data_atualizacao = option_to_string(attr.value()),
                            _ => {}
                        }
                    };

                    let cpf = Cpf::new(cpf_string.to_string());

                    match cpf {
                        Ok(cpf) => {
                            match id.parse::<usize>() {
                                Ok(id_value) => {
                                    let cliente = Cliente::new(
                                        id_value,
                                        nome,
                                        email,
                                        cpf,
                                        data_criacao,
                                        data_atualizacao,
                                    );
                
                                    clientes.push(cliente);
                                },
                                Err(error) => {
                                    println!("Failed to convert string, ID: {}", id);
                                }
                            }
                            
                        },
                        Err(err) => println!("Invalid CPF for user: {}", nome),
                    };
                }
                Ok(clientes)
            }
            Err(err) => {
                println!("Error during aws cognito request: {}", err);
                Err(DomainError::NotFound)
            }

        }
    }

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        let clientes_result = self.get_clientes().await;

        let clientes = match clientes_result {
            Ok(clientes) => clientes,
            Err(err) => {
                println!("Error retrieving clientes");
                return Err(err);
            }
        };

        for cliente in clientes {
            if cpf.0 == cliente.cpf().0 {
                return Ok(cliente);
            }
        }

        Err(DomainError::NotFound)

    }

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError> {
        let clientes_result = self.get_clientes().await;

        let clientes = match clientes_result {
            Ok(clientes) => clientes,
            Err(err) => {
                println!("Error retrieving clientes");
                return Err(err);
            }
        };

        for cliente in clientes {

            if id == *cliente.id() {
                return Ok(cliente);
            }
        }

        Err(DomainError::NotFound)
    }

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        let cpf_string = &cliente.cpf().0;
        // Initialize an empty vector to hold successfully built attributes
        let mut attributes = Vec::new();

        let id = cpf_string.replace(".", "").replace("-", "");
        let string_id: &str = &id;
        // List of attribute specifications
        let attribute_specs = vec![
            ("custom:id", string_id),
            ("custom:nome", cliente.nome()),
            ("custom:email", cliente.email()),
            ("custom:cpf", cpf_string),
            ("custom:data_criacao", cliente.data_criacao()),
            ("custom:data_atualizacao", cliente.data_atualizacao()),
        ];

        // Iterate over attribute specifications
        for (name, value) in attribute_specs {
            // Attempt to build an attribute
            match AttributeType::builder()
                .name(name)
                .value(value)
                .build()
            {
                Ok(attr) => {
                    // Successfully built the attribute, add it to the vector
                    attributes.push(attr);
                },
                Err(err) => {
                    println!("Failed to build attribute {}: {}", name, err);
                }
            }
        }

        let response = self.client
            .admin_create_user()
            .user_pool_id(&self.user_pool_id)
            .username(cpf_string)
            .temporary_password(cpf_string)
            .set_user_attributes(Some(attributes))
            .send()
            .await;

        match response {
            Ok(resp) => {
                println!("Successfully created user: {}", cliente.id());
                Ok(cliente)
            },
            Err(err) => {
                println!("SDK ERROR: {}",err.to_string());
                println!("Failed to create user: {}", cpf_string);
                Err(DomainError::Invalid("Cliente".to_string()))
            }
        }
    }

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        let cpf_string = cpf.0;
        let response = self.client
            .admin_delete_user()
            .user_pool_id(&self.user_pool_id)
            .username(cpf_string.clone())
            .send()
            .await;

        match response {
            Ok(_) => {
                Ok(())
            },
            Err(err) => {
                println!("Failed to delete user: {}", cpf_string);
                Err(DomainError::NotFound)
            }
        }
    }
}

#[cfg(test)]
mock! {
    pub AwsCognitoClient {}

    #[async_trait]
    impl ClienteGateway for AwsCognitoClient {
        async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError>;
        async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError>;
        async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError>;
        async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError>;
        async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError>;
    }
}

#[tokio::test]
async fn test_get_cliente_by_cpf_success() {
    let mut mock_client = MockAwsCognitoClient::new();

    let cliente = Cliente::new(
        1,
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        Cpf::new("123.456.789-00".to_string()).unwrap(),
        "2021-01-01T00:00:00Z".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
    );

    mock_client.expect_get_clientes()
        .returning(move || Ok(vec![cliente.clone()]));

    let result = mock_client.get_cliente_by_cpf(Cpf::new("123.456.789-00".to_string()).unwrap()).await;

    assert!(result.is_ok());
    let retrieved_cliente = result.unwrap();
    assert_eq!(retrieved_cliente.id(), &1);
    assert_eq!(retrieved_cliente.nome(), "John Doe");
    assert_eq!(retrieved_cliente.email(), "john.doe@example.com");
}

#[tokio::test]
async fn test_get_cliente_by_cpf_not_found() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Ok(vec![]));

    let result = mock_client.get_cliente_by_cpf(Cpf::new("123.456.789-00".to_string()).unwrap()).await;

    assert!(matches!(result, Err(DomainError::NotFound)));
}

#[tokio::test]
async fn test_get_cliente_by_cpf_error() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Err(DomainError::NotFound));

    let result = mock_client.get_cliente_by_cpf(Cpf::new("123.456.789-00".to_string()).unwrap()).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(DomainError::NotFound)));
}

#[tokio::test]
async fn test_get_cliente_by_id_found() {
    let mut mock_client = MockAwsCognitoClient::new();

    let cliente = Cliente::new(
        1,
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        Cpf::new("123.456.789-00".to_string()).unwrap(),
        "2021-01-01T00:00:00Z".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
    );

    mock_client.expect_get_clientes()
        .returning(move || Ok(vec![cliente.clone()]));

    let result = mock_client.get_cliente_by_id(1).await;

    assert!(result.is_ok());
    let retrieved_cliente = result.unwrap();
    assert_eq!(retrieved_cliente.id(), &1);
    assert_eq!(retrieved_cliente.nome(), "John Doe");
    assert_eq!(retrieved_cliente.email(), "john.doe@example.com");
}

#[tokio::test]
async fn test_get_cliente_by_id_not_found() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Ok(vec![]));

    let result = mock_client.get_cliente_by_id(1).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(DomainError::NotFound)));
}

#[tokio::test]
async fn test_get_cliente_by_id_error() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Err(DomainError::NotFound));

    let result = mock_client.get_cliente_by_id(1).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(DomainError::NotFound)));
}

#[tokio::test]
async fn test_get_clientes_success() {
    let mut mock_client = MockAwsCognitoClient::new();

    let cliente1 = Cliente::new(
        1,
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        Cpf::new("123.456.789-00".to_string()).unwrap(),
        "2021-01-01T00:00:00Z".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
    );

    let cliente2 = Cliente::new(
        2,
        "Jane Smith".to_string(),
        "jane.smith@example.com".to_string(),
        Cpf::new("987.654.321-00".to_string()).unwrap(),
        "2021-01-02T00:00:00Z".to_string(),
        "2021-01-02T00:00:00Z".to_string(),
    );

    mock_client.expect_get_clientes()
        .returning(move || Ok(vec![cliente1.clone(), cliente2.clone()]));

    let result = mock_client.get_clientes().await;

    assert!(result.is_ok());
    let retrieved_clientes = result.unwrap();
    assert_eq!(retrieved_clientes.len(), 2);
    assert_eq!(retrieved_clientes[0].id(), &1);
    assert_eq!(retrieved_clientes[1].id(), &2);
}

#[tokio::test]
async fn test_get_clientes_empty() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Ok(vec![]));

    let result = mock_client.get_clientes().await;

    assert!(result.is_ok());
    let retrieved_clientes = result.unwrap();
    assert!(retrieved_clientes.is_empty());
}

#[tokio::test]
async fn test_get_clientes_error() {
    let mut mock_client = MockAwsCognitoClient::new();

    mock_client.expect_get_clientes()
        .returning(|| Err(DomainError::NotFound));

    let result = mock_client.get_clientes().await;

    assert!(result.is_err());
    assert!(matches!(result, Err(DomainError::NotFound)));
}

#[tokio::test]
async fn test_create_cliente_success() {
    let mut mock_client = MockAwsCognitoClient::new();

    let cliente = Cliente::new(
        1,
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        Cpf::new("123.456.789-00".to_string()).unwrap(),
        "2021-01-01T00:00:00Z".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
    );

    mock_client.expect_create_cliente()
        .with(eq(cliente.clone()))
        .returning(|cliente| Ok(cliente));

    let result = mock_client.create_cliente(cliente.clone()).await;

    assert!(result.is_ok());
    let created_cliente = result.unwrap();
    assert_eq!(created_cliente.id(), &1);
    assert_eq!(created_cliente.nome(), "John Doe");
    assert_eq!(created_cliente.email(), "john.doe@example.com");
}

#[tokio::test]
async fn test_create_cliente_error() {
    let mut mock_client = MockAwsCognitoClient::new();

    let cliente = Cliente::new(
        1,
        "John Doe".to_string(),
        "john.doe@example.com".to_string(),
        Cpf::new("123.456.789-00".to_string()).unwrap(),
        "2021-01-01T00:00:00Z".to_string(),
        "2021-01-01T00:00:00Z".to_string(),
    );

    mock_client.expect_create_cliente()
        .with(eq(cliente.clone()))
        .returning(|_| Err(DomainError::Invalid("Error creating cliente".to_string())));

    let result = mock_client.create_cliente(cliente.clone()).await;

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DomainError::Invalid("Error creating cliente".to_string()));
}
