use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::{
    base::{
        assertion_concern,
        domain_error::DomainError,
    },
    entities::cpf::Cpf,
};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum Status {
    Ativo,
    Inativo,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum Tipo {
    Admin,
    Cozinha,
}

impl FromStr for Tipo {
    type Err = ();

    fn from_str(input: &str) -> Result<Tipo, Self::Err> {
        match input {
            "Admin" => Ok(Tipo::Admin),
            "Cozinha" => Ok(Tipo::Cozinha),
            _ => Err(()),
        }
    }
}


impl fmt::Display for Tipo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tipo::Admin => "Admin",
                Tipo::Cozinha => "Cozinha",
            }
        )
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Usuario {
    id: usize,
    nome: String,
    email: String,
    cpf: Cpf,
    #[serde(skip_serializing)]
    senha: String,
    tipo: Tipo,
    status: Status,
    data_criacao: String,
    data_atualizacao: String,
}

impl Usuario {
    pub fn new(
        id: usize,
        nome: String,
        email: String,
        cpf: Cpf,
        senha: String,
        tipo: Tipo,
        status: Status,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        Usuario {
            id,
            nome,
            email,
            cpf,
            senha,
            tipo,
            status,
            data_criacao,
            data_atualizacao,
        }
    }

    fn validate_entity(&self) -> Result<(), DomainError> {
        match self.status {
            Status::Ativo | Status::Inativo => (),
            _ => {
                return Err(DomainError::Invalid(
                    "Status do Usuário é inválido".to_string(),
                ))
            }
        };
        match self.tipo {
            Tipo::Admin | Tipo::Cozinha => (),
            _ => {
                return Err(DomainError::Invalid(
                    "Tipo do Usuário é inválido".to_string(),
                ))
            }
        };
        assertion_concern::assert_argument_not_empty(self.nome.clone())?;
        assertion_concern::assert_argument_not_empty(self.email.clone())?;
        assertion_concern::assert_argument_not_empty(self.senha.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_criacao.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_atualizacao.clone())?;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn senha(&self) -> &String {
        &self.senha
    }

    pub fn cpf(&self) -> &Cpf {
        &self.cpf
    }

    pub fn validate_senha(&self, senha: &String) -> bool {
        &self.senha == senha
    }

    pub fn tipo(&self) -> &Tipo {
        &self.tipo
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    // Setters
    pub fn set_nome(&mut self, nome: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(nome.clone())?;
        self.nome = nome;
        Ok(())
    }

    pub fn set_email(&mut self, email: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(email.clone())?;
        self.email = email;
        Ok(())
    }

    pub fn set_cpf(&mut self, cpf: Cpf) {
        self.cpf = cpf;
    }

    pub fn set_senha(&mut self, senha: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(senha.clone())?;
        self.senha = senha;
        Ok(())
    }

    pub fn set_tipo(&mut self, tipo: Tipo) {
        self.tipo = tipo;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_timestamp_format(data_atualizacao.clone())?;
        self.data_atualizacao = data_atualizacao;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_usuario() -> Usuario {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        Usuario::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "senha_segura".to_string(),
            Tipo::Admin,
            Status::Ativo,
            _now.clone(),
            _now,
        )
    }

    #[test]
    fn test_usuario_creation_valid() {
        let usuario = create_valid_usuario();
        assert_eq!(usuario.id(), &1);
        assert_eq!(usuario.nome(), "Fulano da Silva");
        assert_eq!(usuario.email(), "fulano.silva@exemplo.com");
        assert_eq!(usuario.tipo(), &Tipo::Admin);
        assert_eq!(usuario.status(), &Status::Ativo);
    }

    #[test]
    fn test_usuario_validate_entity_valid() {
        let usuario = create_valid_usuario();
        assert!(usuario.validate_entity().is_ok());
    }

    #[test]
    fn test_usuario_setters_valid() {
        let mut usuario = create_valid_usuario();
        let _ = usuario.set_nome("Ciclano de Almeida".to_string());
        let _ = usuario.set_email("ciclano.almeida@exemplo.com".to_string());
        let _ = usuario.set_senha("nova_senha_segura".to_string());
        usuario.set_tipo(Tipo::Cozinha);
        usuario.set_status(Status::Inativo);
        assert_eq!(usuario.nome(), "Ciclano de Almeida");
        assert_eq!(usuario.email(), "ciclano.almeida@exemplo.com");
        assert!(usuario.validate_senha(&"nova_senha_segura".to_string()));
        assert_eq!(usuario.tipo(), &Tipo::Cozinha);
        assert_eq!(usuario.status(), &Status::Inativo);
    }

    #[test]
    fn test_usuario_set_nome_empty() {
        let mut usuario = create_valid_usuario();
        let result = usuario.set_nome("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_email_empty() {
        let mut usuario = create_valid_usuario();
        let result = usuario.set_email("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_senha_empty() {
        let mut usuario = create_valid_usuario();
        let result = usuario.set_senha("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_data_atualizacao_invalid_format() {
        let mut usuario = create_valid_usuario();
        let result = usuario.set_data_atualizacao("18-02-2024".to_string());
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }
}
