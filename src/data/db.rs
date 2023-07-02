use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    Write,
    Delete,
}

pub trait SqliteManager {
    type Item: serde::de::DeserializeOwned + serde::Serialize;

    fn table_name() -> String;

    // fn select(key: u32) -> Result<Self::Item, CRUDError> {
    //     let conn = Self::connect();
    //     let sql = format!("SELECT * FROM {} WHERE id = ?", Self::table_name());
    //     let mut stmt = conn.prepare(&sql)?;
    //     let row = stmt.query_row(&[key], |row| row.to_owned())?;
    //     let item: Self::Item = serde_json::from_str(&row.to_json())?;
    //     Ok(item)
    // }
    fn read(key: u32) -> Result<Vec<Self::Item>, CRUDError> {
        let conn = Self::connect();
        let query = format!("SELECT * FROM {}", Self::table_name());
        let mut stmt = conn.prepare(&query).unwrap();

        let rows = stmt.query_map([], |row| row.get("title")).unwrap();

        // let mut result = Vec::new();

        // for row in rows {
        //     if let Ok(row) = row {
        //         result.push(row);
        //     }
        // }
        return Err(CRUDError::Write);
    }

    // fn insert(key: u32, value: &Self::Item) -> Result<(), CRUDError> {
    //     let conn = Self::connect();
    //     let sql = format!(
    //         "INSERT OR REPLACE INTO {} (id, data) VALUES (?, ?)",
    //         Self::table_name()
    //     );
    //     let json_value = serde_json::to_string(value)?;
    //     conn.execute(&sql, &[&key, &json_value])
    //         .map_err(|err| {
    //             eprintln!("Failed to set item data: {:?}", err);
    //             CRUDError::Write
    //         })
    //         .map(|_| ())
    // }
    // fn update(table: String, model_data: Value, id: i16) -> i16 {
    //     let fields_names = get_model_fields_information_for_update(model_data);
    //     let connection = connect();
    //     let (message, result) = match connection.execute(
    //         &format!("UPDATE {table} SET {fields_names} WHERE id = {id}"),
    //         (),
    //     ) {
    //         Ok(updated) => (println!("{} rows were updated", updated), id),
    //         Err(err) => (println!("update failed: {}", err), 0),
    //     };
    //     return result;
    // }

    fn delete(table: String, id: i16) -> Result<i16, CRUDError> {
        let connection = Self::connect();
        match connection.execute(&format!("DELETE FROM {table} WHERE id = {id}"), ()) {
            Ok(deleted) => {
                println!("{} rows were deleted", deleted);
                Ok(id)
            }
            Err(err) => Err(CRUDError::Delete),
        }
    }

    fn connect() -> Connection {
        Connection::open_in_memory().unwrap()
    }
}
