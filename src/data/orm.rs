use std::marker::PhantomData;

pub struct Select;
pub struct From;
pub struct Where;
pub struct Join;
pub struct Limit;

pub struct Orm<State = Select> {
    query: String,
    state: PhantomData<State>,
    has_where_clause: bool,
}

impl Orm {
    pub fn new() -> Orm<Select> {
        Orm {
            query: String::new(),
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

impl Orm<Select> {
    pub fn select(self, columns: &str) -> Orm<From> {
        Orm {
            query: format!("SELECT {}", columns),
            has_where_clause: self.has_where_clause,
            state: PhantomData,
        }
    }
    pub fn select_single(self, columns: &str) -> String {
        format!("SELECT {};", columns)
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
