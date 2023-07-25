use std::marker::PhantomData;

use sqlx::{migrate::MigrateDatabase, sqlite::SqliteConnection, Connection, Executor, Sqlite};

use envy::get_env;

pub struct Select;
pub struct From;
pub struct Where;
pub struct Join;
pub struct Limit;
pub struct Table;

impl Orm<Table> {
    pub fn drop_table(self, table: &str) -> Orm<Table> {
        Orm {
            query: format!("DROP TABLE {table}"),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn truncate_table(self, table: &str) -> Orm<Table> {
        Orm {
            query: format!("TRUNCATE TABLE {table}"),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn alter_table(self, table: &str) -> Orm<Table> {
        Orm {
            query: format!("ALTER TABLE {table}"),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn add(self, column: &str, data_type: &str) -> Orm<Table> {
        //TODO: convert data type into enum
        Orm {
            query: format!("{} ADD {column} {data_type}", self.query),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
}

pub struct Orm<State = Select> {
    query: String,
    state: PhantomData<State>,
    has_where_clause: bool,
}

impl Orm {
    pub fn select(columns: &str) -> Orm<From> {
        Orm {
            query: format!("SELECT {}", columns),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn select_single(columns: &str) -> String {
        format!("SELECT {};", columns)
    }

    pub fn insert(columns: &str) -> Orm<Select> {
        Orm {
            query: format!("INSERT {}", columns),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn delete(columns: &str) -> Orm<Where> {
        Orm {
            query: format!("DELETE {}", columns),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn update(columns: &str) -> Orm<Where> {
        Orm {
            query: format!("UPDATE {}", columns),
            has_where_clause: false,
            state: PhantomData,
        }
    }
}

impl<State> Orm<State> {
    pub fn ready(&mut self) -> String {
        self.query.push_str(";");
        self.query.clone()
    }
}

impl Orm<From> {
    pub fn from(self, table: &str) -> Orm<From> {
        Orm {
            query: format!("{} FROM {}", self.query, table),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
    pub fn from_subquery(self, subquery: &str) -> Orm<From> {
        Orm {
            query: format!("{} FROM {}", self.query, subquery),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
    pub fn join(self, table: &str) -> Orm<Join> {
        Orm {
            query: format!("{} JOIN {}", self.query, table),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
    pub fn where_clause(self) -> Orm<Where> {
        Orm {
            query: format!("{} WHERE ", self.query),
            has_where_clause: false,
            state: PhantomData,
        }
    }
}

impl Orm<Join> {
    pub fn join(self, table: &str) -> Orm<Join> {
        Orm {
            query: format!("{} JOIN {}", self.query, table),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
    pub fn on(self, joint: &str) -> Orm<Join> {
        Orm {
            query: format!("{} ON {}", self.query, joint),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
}

impl Orm<Where> {
    pub fn where_clause(&mut self, condition: &str) -> Orm<Where> {
        Orm {
            query: format!("{} WHERE {}", self.query, condition),
            has_where_clause: true,
            state: PhantomData,
        }
    }

    pub fn equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} = {}", column, Self::correct_value(value)));
        self
    }

    pub fn not_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} != {}", column, Self::correct_value(value)));
        self
    }

    pub fn less_than(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} < {}", column, Self::correct_value(value)));
        self
    }

    pub fn greater_than(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} > {}", column, Self::correct_value(value)));
        self
    }

    pub fn less_than_or_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} <= {}", column, Self::correct_value(value)));
        self
    }

    pub fn greater_than_or_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} >= {}", column, Self::correct_value(value)));
        self
    }

    pub fn like(&mut self, column: &str, pattern: &str) -> &mut Self {
        self.and();
        self.query.push_str(&format!("{} LIKE {}", column, pattern));
        self
    }

    pub fn not_like(&mut self, column: &str, pattern: &str) -> &mut Self {
        self.and();
        self.query
            .push_str(&format!("{} NOT LIKE {}", column, pattern));
        self
    }

    pub fn in_values(&mut self, column: &str, values: &[&str]) -> &mut Self {
        self.and();
        let value_list = values
            .iter()
            .map(|value| format!("{}", Self::correct_value(value)))
            .collect::<Vec<String>>()
            .join(", ");
        self.query
            .push_str(&format!("{} IN ({})", column, value_list));
        self
    }

    pub fn not_in_values(&mut self, column: &str, values: &[&str]) -> &mut Self {
        self.and();
        let value_list = values
            .iter()
            .map(|value| format!("{}", Self::correct_value(value)))
            .collect::<Vec<String>>()
            .join(", ");
        self.query
            .push_str(&format!("{} NOT IN ({})", column, value_list));
        self
    }

    pub fn and(&mut self) -> &mut Self {
        if self.has_where_clause {
            self.query.push_str(" AND ");
        } else {
            self.has_where_clause = true;
        }
        self
    }
    fn correct_value(value: &str) -> String {
        if value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok() {
            return value.to_string();
        }
        return format!("{}", value.to_owned());
    }
}

impl Orm<Limit> {
    pub fn limit(&mut self, limit: u32) -> Orm<Limit> {
        Orm {
            query: format!("{} LIMIT {}", self.query, limit),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
}

pub async fn run_migrations(file_path: &str) {
    let db_url = get_env("DATABASE_URL");
    match Sqlite::create_database(&db_url).await {
        Ok(_) => println!("database created"),
        Err(err) => {
            println!("error creating db:  {err}");
            return;
        }
    };
    // Open a connection to the SQLite database
    let mut connection = match SqliteConnection::connect(&db_url).await {
        Ok(connection) => connection,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // Read the SQL file contents
    let sql = match tokio::fs::read_to_string(file_path).await {
        Ok(sql) => sql,
        Err(err) => {
            println!("error reading files:  {err}");
            return;
        }
    };

    match connection.execute(&*sql).await {
        Ok(result) => println!("succesfull result: {:?}", result),
        Err(err) => {
            println!("error executing sql: {err}");
            return;
        }
    };
}
