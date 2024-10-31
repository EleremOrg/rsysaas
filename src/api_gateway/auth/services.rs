use jsonwebtoken::EncodingKey;
use stefn::{create_token, verify_password, AppError, Database};

use super::PrivateClaims;

pub async fn get_token(
    database: &Database,
    encoding_keys: &EncodingKey,
    username: &str,
    password: &str,
) -> Result<String, AppError> {
    let user = find_user_password(database, username).await?;
    //TODO: finish
    tracing::info!("{:?}", &user);
    verify_password(password, &user.password)?;
    create_token(
        user.pk,
        PrivateClaims::new(user.groups, user.company_pk),
        &user.domain,
        encoding_keys,
    )
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    pk: i64,
    password: String,
    company_pk: i64,
    domain: String,
    groups: String,
}

async fn find_user_password(database: &Database, username: &str) -> Result<User, AppError> {
    let result: Option<User> =
        sqlx::query_as(r#"
            SELECT users.pk, users.password, customers_companies.pk as company_pk, customers_companies.domain, GROUP_CONCAT(groups.name, ', ') AS groups
            FROM users
            INNER JOIN customers ON users.pk = customers.user_pk
            INNER JOIN customers_companies ON customers_companies.pk = customers.pk
            LEFT JOIN users_groups_m2m ON users.pk = users_groups_m2m.user_pk
            LEFT JOIN groups ON users_groups_m2m.group_pk = groups.pk
            WHERE users.username = $1
            HAVING count(users.pk) > 0;
        "#)
            .bind(username)
            .fetch_optional(database.get_connection().await)
            .await
            .map_err(|e| AppError::custom_internal("error during desiarialization"))?;
    result.ok_or(AppError::DoesNotExist)
}
