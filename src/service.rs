use crate::{errors::RBACError, repository::RbacRepository, types::Id};
pub struct RbacService {
    repo: RbacRepository,
}

impl RbacService {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let repo = RbacRepository::new(database_url).await?;
        Ok(Self { repo })
    }


    pub async fn assign_role(&self, user_id: Id, role_id: Id) -> Result<(), Box<dyn std::error::Error>> {
        self.repo.assign_role_to_user(user_id, role_id).await?;
        Ok(())
    }

    pub async fn check_permission(&self, user_id: Id, permission: &str) -> Result<bool, RBACError> {
        self.repo.user_has_permission(user_id, permission).await
    }
}
