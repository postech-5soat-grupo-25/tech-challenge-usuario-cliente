use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::{
    cpf::Cpf,
    usuario::{Status, Usuario},
};
use std::fmt;
use std::str::FromStr;



#[automock]
#[async_trait]
pub trait UsuarioGateway {
    async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError>;

    async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError>;

    async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError>;

    async fn create_usuario(&mut self, user: Usuario) -> Result<Usuario, DomainError>;

    async fn update_usuario(
        &mut self,
        dados_usuario_atualizado: Usuario,
    ) -> Result<Usuario, DomainError>;

    async fn delete_usuario(&mut self, cpf: Cpf) -> Result<(), DomainError>;
}
