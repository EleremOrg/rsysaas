use sqlx::{QueryBuilder, Sqlite};
use stefn::{AppError, AppState};

use super::models::products::ProductCategory;

pub async fn run_transaction(state: AppState, payload: ProductCategory) -> Result<u64, AppError> {
    let mut tx = state
        .primary_database
        .begin()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    let query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO products(id, company_pk, meta) ");
    let company_pk = 1;
    let mut query_builder: QueryBuilder<Sqlite> =
        complete_query(payload, company_pk, query_builder);

    let result = query_builder
        .build()
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?
        .rows_affected();

    let _ = tx
        .commit()
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?;
    Ok(result)
}

fn complete_query(
    payload: ProductCategory,
    company_pk: i64,
    mut query_builder: QueryBuilder<Sqlite>,
) -> QueryBuilder<Sqlite> {
    match payload {
        ProductCategory::Clothing(products) => {
            query_builder.push_values(products, |mut b, product| {
                b.push_bind(product.id.clone())
                    .push_bind(company_pk)
                    .push_bind(sqlx::types::Json(product));
            });
        }
        ProductCategory::SportsAndOutdoors(products) => {
            query_builder.push_values(products, |mut b, product| {
                b.push_bind(product.id.clone())
                    .push_bind(company_pk)
                    .push_bind(sqlx::types::Json(product));
            });
        }
        ProductCategory::BooksAndMedia(products) => {
            query_builder.push_values(products, |mut b, product| {
                b.push_bind(product.id.clone())
                    .push_bind(company_pk)
                    .push_bind(sqlx::types::Json(product));
            });
        }
    };
    query_builder
}
