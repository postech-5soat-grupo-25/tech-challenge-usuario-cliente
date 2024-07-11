use std::str::FromStr;

use aws_config::from_env;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::operation::list_users;
use aws_sdk_cognitoidentityprovider::types::AttributeType;
use aws_sdk_cognitoidentityprovider::error::SdkError;
use aws_sdk_cognitoidentityprovider::error::UnknownVariantError;
use aws_sdk_cognitoidentityprovider::{config::Region, meta::PKG_VERSION, Client};
use chrono::Utc;

use crate::base::domain_error::DomainError;
use crate::{
    traits::usuario_gateway::UsuarioGateway,
};

use crate::entities::{
    cpf::Cpf,
    usuario::{Status, Usuario,Tipo},
};

fn option_to_string(option: Option<&str>) -> String {
    match option {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}


pub struct AwsCognitoUsuarioRepository {
    client: Client,
    user_pool_id: String,
}

impl AwsCognitoUsuarioRepository {
    pub async fn new(user_pool_id: String) -> Self {
        let region_provider = RegionProviderChain::default_provider();

        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);
        

        let mut repo = AwsCognitoUsuarioRepository {
            client,
            user_pool_id,
        };

        repo.check_for_usuario_admin().await;

        repo
    }

    async fn check_for_usuario_admin(&mut self) {
        let admin_cpf = Cpf::new("000.000.000-00".to_string()).unwrap();
        let usuario_admin = self.get_usuario_by_cpf(admin_cpf).await;
        match usuario_admin {
            Ok(usuario) => {
                println!("Usuário Admin encontrado: {:?}", usuario);
            }
            _ => {
                println!("Usuário Admin não encontrado. Criando...");
                let _id = 0;
                let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
                let cpf = Cpf::new("000.000.000-00".to_string()).unwrap();
                let usuario_admin = Usuario::new(
                    _id,
                    "Administrador".to_string(),
                    "admin@fastfood.com.br".to_string(),
                    cpf,
                    "melhor_projeto".to_string(),
                    Tipo::Admin,
                    Status::Ativo,
                    _now.clone(),
                    _now,
                );
                self.create_usuario(usuario_admin).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl UsuarioGateway for AwsCognitoUsuarioRepository {
    async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError> {
        let response = self
            .client
            .list_users()
            .user_pool_id(&self.user_pool_id)
            .send()
            .await;

        let mut usuarios: Vec<Usuario> = Vec::new();

        match response {
            Ok(response) => {
                let users = response.users();

                for user in users {
                    let mut id = String::new();
                    let mut nome = String::new();
                    let mut email = String::new();
                    let mut cpf_string = String::new();
                    let mut senha = String::new();
                    let mut tipo_string = String::new();
                    let mut status_string = String::new();
                    let mut data_criacao = String::new();
                    let mut data_atualizacao = String::new();

                    for attr in user.attributes() {
                        match attr.name() {
                            "custom:id" => id =option_to_string(attr.value()),
                            "custom:nome" => nome = option_to_string(attr.value()),
                            "custom:email" => email = option_to_string(attr.value()),
                            "custom:cpf" => cpf_string = option_to_string(attr.value()),
                            "custom:senha" => senha = option_to_string(attr.value()),
                            "custom:tipo" => tipo_string = option_to_string(attr.value()),
                            "custom:status" => status_string = option_to_string(attr.value()),
                            "custom:data_criacao" => data_criacao = option_to_string(attr.value()),
                            "custom:data_atualizacao" => data_atualizacao = option_to_string(attr.value()),
                            _ => {}
                        }
                    };

                    let cpf = Cpf::new(cpf_string.to_string());

                    let status = Status::from_str(status_string.as_str());

                    if cpf.is_err() {
                        println!("Error on cpf skiping");
                        continue
                    }

                    let cpf = cpf.unwrap();

                    let tipo = match Tipo::from_str(tipo_string.as_str()) {
                        Ok(tipo) => tipo,
                        Err(_) => continue, // Skip iteration if Tipo is invalid
                    };

                    let status = match Status::from_str(status_string.as_str()) {
                        Ok(status) => status,
                        Err(_) => continue, // Skip iteration if Status is invalid
                    };

                    match id.parse::<usize>() {
                        Ok(id_value) => {
                            let Usuario = Usuario::new(
                                id_value,
                                nome,
                                email,
                                cpf,
                                senha,
                                tipo,
                                status,
                                data_criacao,
                                data_atualizacao,
                            );
        
                            usuarios.push(Usuario);
                        },
                        Err(error) => {
                            println!("Failed to convert string, ID: {}", id);
                        }
                    }
                }
                Ok(usuarios)
            }
            Err(SdkError::ServiceError(err)) => {
                println!("Service error: {:?}", err);
                println!("Service error details: {:?}", err);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::TimeoutError(source)) => {
                println!("Timeout error: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::DispatchFailure (source)) => {
                println!("Dispatch failure: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::ResponseError (source)) => {
                println!("Response error: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(err) => {
                println!("Other SDK error: {:?}", err);
                Err(DomainError::Invalid("Usuario".to_string()))
            }

        }
    }

    async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        let usuario_result = self.get_usuarios().await;

        let usuarios = match usuario_result {
            Ok(usuarios) => usuarios,
            Err(err) => {
                println!("Error retrieving Usuarios");
                return Err(err);
            }
        };

        for usuario in usuarios {
            if cpf.0 == usuario.cpf().0 {
                return Ok(usuario);
            }
        }

        Err(DomainError::NotFound)

    }

    async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        let usuarios_result = self.get_usuarios().await;

        let usuarios = match usuarios_result {
            Ok(usuarios) => usuarios,
            Err(err) => {
                println!("Error retrieving Usuarios");
                return Err(err);
            }
        };

        for usuario in usuarios {

            if id == *usuario.id() {
                return Ok(usuario);
            }
        }

        Err(DomainError::NotFound)
    }

    async fn create_usuario(&mut self, usuario: Usuario) -> Result<Usuario, DomainError> {
        let cpf_string = &usuario.cpf().0;
        // Initialize an empty vector to hold successfully built attributes
        let mut attributes = Vec::new();
    
        let id = cpf_string.replace(".", "").replace("-", "");
        let string_id: &str = &id;
        let tipo = &usuario.tipo().to_string();
        let status = &usuario.status().to_string();
        // List of attribute specifications
        let attribute_specs = vec![
            ("custom:id", string_id),
            ("custom:nome", usuario.nome()),
            ("custom:email", usuario.email()),
            ("custom:cpf", cpf_string),
            ("custom:senha", usuario.senha()),
            ("custom:tipo", tipo),
            ("custom:status", status),
            ("custom:data_criacao", usuario.data_criacao()),
            ("custom:data_atualizacao", usuario.data_atualizacao()),
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
                println!("Successfully created user: {}", usuario.id());
                Ok(usuario)
            },
            Err(SdkError::ServiceError(err)) => {
                println!("Service error: {:?}", err);
                println!("Service error details: {:?}", err);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::TimeoutError(source)) => {
                println!("Timeout error: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::DispatchFailure (source)) => {
                println!("Dispatch failure: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(SdkError::ResponseError (source)) => {
                println!("Response error: {:?}", source);
                Err(DomainError::Invalid("Usuario".to_string()))
            },
            Err(err) => {
                println!("Other SDK error: {:?}", err);
                Err(DomainError::Invalid("Usuario".to_string()))
            }
        }
    }

    async fn update_usuario(&mut self, dados_usuario_atualizado: Usuario) -> Result<Usuario, DomainError> {
        let cpf_string = dados_usuario_atualizado.cpf().0.clone();
        let id = cpf_string.replace(".", "").replace("-", "");
        let string_id: &str = &id;
        let tipo = dados_usuario_atualizado.tipo().to_string().clone();
        let status = dados_usuario_atualizado.status().to_string().clone();

        // List of attribute specifications
        let attribute_specs = vec![
            ("custom:id", string_id),
            ("custom:nome", dados_usuario_atualizado.nome()),
            ("custom:email", dados_usuario_atualizado.email()),
            ("custom:cpf", cpf_string.as_str()),
            ("custom:senha", dados_usuario_atualizado.senha()),
            ("custom:tipo", tipo.as_str()),
            ("custom:status", status.as_str()),
            ("custom:data_criacao", dados_usuario_atualizado.data_criacao()),
            ("custom:data_atualizacao", dados_usuario_atualizado.data_atualizacao()),
        ];

        // Initialize an empty vector to hold successfully built attributes
        let mut attributes = Vec::new();

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

        // Send request to update user attributes
        let response = self.client
            .admin_update_user_attributes()
            .user_pool_id(&self.user_pool_id)
            .username(cpf_string.as_str())
            .set_user_attributes(Some(attributes))
            .send()
            .await;

        match response {
            Ok(_) => {
                println!("Successfully updated user: {}", dados_usuario_atualizado.id());
                Ok(dados_usuario_atualizado)
            },
            Err(err) => {
                println!("SDK ERROR: {}", err.to_string());
                println!("Failed to update user: {}", cpf_string);
                Err(DomainError::Invalid("Usuario".to_string()))
            }
        }
    }

    async fn delete_usuario(&mut self, cpf: Cpf) -> Result<(), DomainError> {
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
