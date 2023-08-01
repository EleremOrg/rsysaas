use std::marker::PhantomData;

pub struct Select;
pub struct From;
pub struct Where;
pub struct Join;
pub struct Limit;
pub struct Table;
pub struct Insert;

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

    pub fn insert(table: &str) -> Orm<Insert> {
        Orm {
            query: format!("INSERT INTO {table}"),
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
        if self.query.ends_with(",") {
            self.query.pop();
        }
        self.query.push_str(";");
        self.query.clone()
    }

    pub fn as_(self, short: &str) -> Orm<State> {
        Orm {
            query: format!("{} AS {}", self.query, short),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }

    pub fn and(self) -> Orm<State> {
        Orm {
            query: format!("{} AND", self.query),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }

    pub fn where_(&mut self) -> Orm<Where> {
        Orm {
            query: format!("{} WHERE", self.query),
            has_where_clause: false,
            state: PhantomData,
        }
    }
}

impl Orm<Insert> {
    pub fn set_columns(self, columns: &str) -> Orm<Insert> {
        Orm {
            query: format!("{} ({columns}) VALUES", self.query),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn add_value(self, values: &str) -> Orm<Insert> {
        Orm {
            query: format!("{} ({values}),", self.query),
            has_where_clause: false,
            state: PhantomData,
        }
    }

    pub fn add_many(self, values: &str) -> Orm<Insert> {
        Orm {
            query: format!("{} {values}", self.query),
            has_where_clause: false,
            state: PhantomData,
        }
    }
}

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
}

impl Orm<Join> {
    pub fn join(self, table: &str) -> Orm<Join> {
        Orm {
            query: format!("{} JOIN {}", self.query, table),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }

    pub fn left_join(self, table: &str) -> Orm<Join> {
        Orm {
            query: format!("{} LEFT JOIN {}", self.query, table),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }

    pub fn on(self, on_clause: &str) -> Orm<Join> {
        Orm {
            query: format!("{} ON {}", self.query, on_clause),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
}

impl Orm<Where> {
    pub fn equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!(" {} = {}", column, Self::correct_value(value)));
        self
    }

    pub fn not_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!(" {} != {}", column, Self::correct_value(value)));
        self
    }

    pub fn less_than(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!("{} < {}", column, Self::correct_value(value)));
        self
    }

    pub fn greater_than(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!("{} > {}", column, Self::correct_value(value)));
        self
    }

    pub fn less_than_or_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!("{} <= {}", column, Self::correct_value(value)));
        self
    }

    pub fn greater_than_or_equal(&mut self, column: &str, value: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!("{} >= {}", column, Self::correct_value(value)));
        self
    }

    pub fn like(&mut self, column: &str, pattern: &str) -> &mut Self {
        self.and_for_where();
        self.query.push_str(&format!("{} LIKE {}", column, pattern));
        self
    }

    pub fn not_like(&mut self, column: &str, pattern: &str) -> &mut Self {
        self.and_for_where();
        self.query
            .push_str(&format!("{} NOT LIKE {}", column, pattern));
        self
    }

    pub fn in_values(&mut self, column: &str, values: &[&str]) -> &mut Self {
        self.and_for_where();
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
        self.and_for_where();
        let value_list = values
            .iter()
            .map(|value| format!("{}", Self::correct_value(value)))
            .collect::<Vec<String>>()
            .join(", ");
        self.query
            .push_str(&format!("{} NOT IN ({})", column, value_list));
        self
    }

    pub fn and_for_where(&mut self) -> &mut Self {
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
        return format!("'{}'", value.to_owned());
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
