use sqlx::{Pool, Postgres};

use crate::{errors::RBACError, models::Role, types::Id};

pub struct RbacRepository {
    pool: Pool<Postgres>,
}

impl RbacRepository {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = Pool::<Postgres>::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn create_role(&self, name: &str) -> Result<Role, RBACError> {
        let role = sqlx::query_as!(
            Role,
            "INSERT INTO roles (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(role)
    }

    pub async fn assign_role_to_user(&self, user_id: Id, role_id: Id) -> Result<(), RBACError> {
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            user_id,
            role_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn user_has_role(&self, user_id: Id, role_name: &str) -> Result<bool, RBACError> {
        let result = sqlx::query!(
            "SELECT roles.id FROM roles
             INNER JOIN user_roles ON roles.id = user_roles.role_id
             WHERE user_roles.user_id = $1 AND roles.name = $2",
            user_id,
            role_name
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(result.is_some())
    }

    pub async fn assign_permission_to_role(&self, role_id: Id, permission_id: Id) -> Result<(), RBACError> {
        sqlx::query!(
            "INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            role_id,
            permission_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn user_has_permission(&self, user_id: Id, permission_name: &str) -> Result<bool, RBACError> {
        let result = sqlx::query!(
            "SELECT permissions.id FROM permissions
             INNER JOIN role_permissions ON permissions.id = role_permissions.permission_id
             INNER JOIN user_roles ON role_permissions.role_id = user_roles.role_id
             WHERE user_roles.user_id = $1 AND permissions.name = $2",
            user_id,
            permission_name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.is_some())
    }
}
