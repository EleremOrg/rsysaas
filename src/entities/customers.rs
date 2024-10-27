use std::marker::PhantomData;

use sqlx::SqliteConnection;
use stefn::AppError;

pub struct Empty;
pub struct User;
pub struct Customer;

pub struct Builder<'a, T = Empty> {
    con: sqlx::Transaction<'a, sqlx::Sqlite>,
    user: Option<UserBuilder>,
    customer: Option<CustomerBuilder>,
    phantom: PhantomData<T>,
}

impl<'a> Builder<'a> {
    pub fn new(con: sqlx::Transaction<'a, sqlx::Sqlite>) -> Builder<User> {
        Builder {
            con,
            user: None,
            customer: None,
            phantom: PhantomData,
        }
    }
}

impl<'a, User> Builder<'a, User> {
    pub async fn user_from_password(
        mut self,
        password: &'a str,
    ) -> Result<Builder<User>, AppError> {
        let user = UserBuilder::new(password, &mut self.con).await?;
        Ok(Builder {
            con: self.con,
            user: Some(user),
            customer: None,
            phantom: PhantomData,
        })
    }

    pub async fn add_email(mut self, email: &str) -> Result<Self, AppError> {
        self.user
            .as_mut()
            .unwrap()
            .add_email(email, &mut self.con)
            .await?;
        Ok(self)
    }

    pub async fn add_to_admin_group(mut self) -> Result<Self, AppError> {
        self.user
            .as_mut()
            .unwrap()
            .add_to_group(1, &mut self.con)
            .await?;
        Ok(self)
    }

    pub async fn customer_from_names(
        mut self,
        first_name: &str,
        last_name: &str,
    ) -> Result<Builder<'a, Customer>, AppError> {
        let customer = CustomerBuilder::new(
            first_name,
            last_name,
            self.user.as_ref().unwrap().pk,
            &mut self.con,
        )
        .await?;

        Ok(Builder {
            con: self.con,
            user: self.user,
            customer: Some(customer),
            phantom: PhantomData,
        })
    }
}

impl<'a, Customer> Builder<'a, Customer> {
    pub async fn add_to_new_company(
        mut self,
        name: &str,
        domain: &str,
        description: &str,
        country_code: &str,
        currency_code: &str,
    ) -> Result<Builder<'a, Customer>, AppError> {
        let company_pk = self
            .create_new_company(name, domain, description, country_code, currency_code)
            .await?;
        self.customer
            .as_mut()
            .unwrap()
            .add_to_company(company_pk, &mut self.con)
            .await?;
        Ok(self)
    }

    async fn create_new_company(
        &mut self,
        name: &str,
        domain: &str,
        description: &str,
        country_code: &str,
        currency_code: &str,
    ) -> Result<i64, AppError> {
        let company_pk =
        sqlx::query("INSERT INTO customers_companies(name, domain, description, country_pk, currency_pk) VALUES ($1, $2, $3, (SELECT pk FROM countries WHERE code = $4), (SELECT pk FROM countries WHERE code = $5))")
            .bind(name)
            .bind(domain)
            .bind(description)
            .bind(country_code)
            .bind(currency_code)
            .execute(&mut *self.con)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();
        Ok(company_pk)
    }

    pub fn release(self) -> sqlx::Transaction<'a, sqlx::Sqlite> {
        self.con
    }
}

pub struct CustomerBuilder {
    pk: i64,
    company_pk: Option<i64>,
}

impl CustomerBuilder {
    pub async fn new(
        first_name: &str,
        last_name: &str,
        user_pk: i64,
        tx: &mut SqliteConnection,
    ) -> Result<Self, AppError> {
        let pk = sqlx::query(
            "INSERT INTO customers(first_name, last_name, user_pk) VALUES ($1, $2, $3)",
        )
        .bind(first_name)
        .bind(last_name)
        .bind(user_pk)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::custom_internal(&e.to_string()))?
        .last_insert_rowid();
        Ok(Self {
            pk,
            company_pk: None,
        })
    }

    pub async fn add_to_company(
        &mut self,
        company_pk: i64,
        tx: &mut SqliteConnection,
    ) -> Result<&Self, AppError> {
        sqlx::query("INSERT INTO customers_companies_m2m(customer_pk, company_pk) VALUES ($1, $2)")
            .bind(self.pk)
            .bind(company_pk)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?;
        self.company_pk = Some(company_pk);
        Ok(self)
    }
}

pub struct UserBuilder {
    pk: i64,
    emails: Vec<i64>,
}

impl UserBuilder {
    pub async fn new(password: &str, tx: &mut SqliteConnection) -> Result<Self, AppError> {
        let user_pk = sqlx::query("INSERT INTO users(password) VALUES ($1)")
            .bind(password)
            .execute(tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();
        Ok(Self {
            pk: user_pk,
            emails: Vec::with_capacity(3),
        })
    }

    pub async fn add_email(
        &mut self,
        email: &str,
        tx: &mut SqliteConnection,
    ) -> Result<&Self, AppError> {
        let email_pk = sqlx::query("INSERT INTO emails(user_pk, email) VALUES ($1, $2)")
            .bind(self.pk)
            .bind(email)
            .execute(tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?
            .last_insert_rowid();
        self.emails.push(email_pk);
        Ok(self)
    }

    async fn add_to_group(&self, group: i64, tx: &mut SqliteConnection) -> Result<&Self, AppError> {
        sqlx::query("INSERT INTO users_groups_m2m(user_pk, group_pk) VALUES ($1, $2)")
            .bind(self.pk)
            .bind(group)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::custom_internal(&e.to_string()))?;
        Ok(self)
    }
}
