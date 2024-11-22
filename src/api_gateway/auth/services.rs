use jsonwebtoken::EncodingKey;
use stefn::{create_token, verify_password, AppError, Database};

use super::PrivateClaims;

pub async fn generate_token(
    database: &Database,
    encoding_keys: &EncodingKey,
    email: &str,
    password: &str,
    domain: &str,
) -> Result<String, AppError> {
    let user = find_user_by_email(database, email).await?;
    //TODO: finish instead of using the state domain use also the domain from the user
    // it will require to update the validator from the
    tracing::info!("{:?}", &user);
    verify_password(password, &user.password)?;
    create_token(
        user.pk,
        PrivateClaims::new(user.groups, user.company_pk),
        domain,
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

async fn find_user_by_email(database: &Database, email: &str) -> Result<User, AppError> {
    let result: Option<User> =
        sqlx::query_as(r#"
            SELECT emails.user_pk as pk, users.password, customers_companies.pk as company_pk, customers_companies.domain, GROUP_CONCAT(groups.name, ', ') AS groups
            FROM emails
            INNER JOIN users ON emails.user_pk = users.pk
            INNER JOIN customers ON emails.user_pk = customers.user_pk
            INNER JOIN customers_companies ON customers_companies.pk = customers.pk
            LEFT JOIN users_groups_m2m ON emails.user_pk = users_groups_m2m.user_pk
            LEFT JOIN groups ON users_groups_m2m.group_pk = groups.pk
            WHERE emails.email = $1
            HAVING count(emails.user_pk) > 0;
        "#)
            .bind(email)
            .fetch_optional(database.get_connection().await)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    result.ok_or(AppError::DoesNotExist)
}
