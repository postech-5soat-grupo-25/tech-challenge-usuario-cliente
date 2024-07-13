use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::cmp::PartialEq;

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

impl FromStr for Status {
    type Err = ();

    fn from_str(input: &str) -> Result<Status, Self::Err> {
        match input {
            "Ativo" => Ok(Status::Ativo),
            "Inativo" => Ok(Status::Inativo),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Ativo => "Ativo",
                Status::Inativo => "Inativo",
            }
        )
    }
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

impl PartialEq for Usuario {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.nome == other.nome
            && self.email == other.email
            && self.cpf == other.cpf
            && self.senha == other.senha
            && self.tipo == other.tipo
            && self.status == other.status
            && self.data_criacao == other.data_criacao
            && self.data_atualizacao == other.data_atualizacao
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_usuario(now: Option<String>) -> Usuario {
        let now: String = now.unwrap_or_else(|| Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string());
        Usuario::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "senha_segura".to_string(),
            Tipo::Admin,
            Status::Ativo,
            now.clone(),
            now,
        )
    }

    #[test]
    fn test_usuario_creation_valid() {
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let usuario = create_valid_usuario(Some(now.clone()));
        assert_eq!(usuario.id(), &1);
        assert_eq!(usuario.nome(), "Fulano da Silva");
        assert_eq!(usuario.email(), "fulano.silva@exemplo.com");
        assert_eq!(usuario.tipo(), &Tipo::Admin);
        assert_eq!(usuario.status(), &Status::Ativo);
        assert_eq!(usuario.senha(), "senha_segura");
        assert_eq!(usuario.cpf().0, "123.456.789-09".to_string());
        assert_eq!(*usuario.data_criacao(), now.clone());
        assert_eq!(*usuario.data_atualizacao(), now);
    }

    #[test]
    fn test_usuario_validate_entity_valid() {
        let usuario = create_valid_usuario(None);
        assert!(usuario.validate_entity().is_ok());
    }

    #[test]
    fn test_usuario_setters_valid() {
        let mut usuario = create_valid_usuario(None);
        let _ = usuario.set_nome("Ciclano de Almeida".to_string());
        let _ = usuario.set_email("ciclano.almeida@exemplo.com".to_string());
        let _ = usuario.set_senha("nova_senha_segura".to_string());
        usuario.set_tipo(Tipo::Cozinha);
        usuario.set_status(Status::Inativo);
        let _ = usuario.set_cpf(Cpf::new("000.000.000-00".to_string()).unwrap());
        assert_eq!(usuario.nome(), "Ciclano de Almeida");
        assert_eq!(usuario.email(), "ciclano.almeida@exemplo.com");
        assert!(usuario.validate_senha(&"nova_senha_segura".to_string()));
        assert_eq!(usuario.tipo(), &Tipo::Cozinha);
        assert_eq!(usuario.status(), &Status::Inativo);
        assert_eq!(usuario.cpf().0, "000.000.000-00".to_string());
    }

    #[test]
    fn test_usuario_set_nome_empty() {
        let mut usuario = create_valid_usuario(None);
        let result = usuario.set_nome("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_email_empty() {
        let mut usuario = create_valid_usuario(None);
        let result = usuario.set_email("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_senha_empty() {
        let mut usuario = create_valid_usuario(None);
        let result = usuario.set_senha("".to_string());
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_usuario_set_data_atualizacao_valid_format() {
        let mut usuario = create_valid_usuario(None);
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let _ = usuario.set_data_atualizacao(now.clone());
        assert_eq!(*usuario.data_atualizacao(), now);
    }

    #[test]
    fn test_usuario_set_data_atualizacao_invalid_format() {
        let mut usuario = create_valid_usuario(None);
        let result = usuario.set_data_atualizacao("18-02-2024".to_string());
        assert!(
            matches!(result, Err(DomainError::Invalid(_))),
            "Esperado Err(DomainError::Invalid), obtido {:?}",
            result
        );
    }

    #[test]
    fn test_status_from_str() {
        let mut test_status = Status::from_str("Ativo").unwrap();
        assert_eq!(test_status, Status::Ativo);

        test_status = Status::from_str("Inativo").unwrap();
        assert_eq!(test_status, Status::Inativo);

        let test_status = Status::from_str("Invalid");
        assert_eq!(test_status, Err(()));
    }

    #[test]
    fn test_display_ativo() {
        let status = Status::Ativo;
        assert_eq!(status.to_string(), "Ativo");
    }

    #[test]
    fn test_display_inativo() {
        let status = Status::Inativo;
        assert_eq!(status.to_string(), "Inativo");
    }

    #[test]
    fn test_tipo_from_str() {
        // Test valid cases
        let mut test_tipo = Tipo::from_str("Admin").unwrap();
        assert_eq!(test_tipo, Tipo::Admin);

        test_tipo = Tipo::from_str("Cozinha").unwrap();
        assert_eq!(test_tipo, Tipo::Cozinha);

        // Test invalid case
        let test_tipo = Tipo::from_str("Invalid");
        assert_eq!(test_tipo, Err(()));
    }

    #[test]
    fn test_display_admin() {
        let tipo = Tipo::Admin;
        assert_eq!(tipo.to_string(), "Admin");
    }

    #[test]
    fn test_display_cozinha() {
        let tipo = Tipo::Cozinha;
        assert_eq!(tipo.to_string(), "Cozinha");
    }


}
