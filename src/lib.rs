pub mod db {
    #[derive(Debug)]
    pub enum Flag {
        Unique,
        Required,
    }

    #[derive(Debug)]
    pub enum DbPrimitive {
        Int(Option<i32>),
        String(Option<String>),
    }

    impl std::cmp::PartialEq for DbPrimitive {
        fn eq(&self, other: &DbPrimitive) -> bool {
            match self {
                DbPrimitive::Int(Some(v)) => match other {
                    DbPrimitive::Int(Some(vo)) => (v == vo),
                    _ => false,
                },
                DbPrimitive::String(Some(v)) => match other {
                    DbPrimitive::String(Some(vo)) => v == vo,
                    _ => false,
                },
                _ => false,
            }
        }
    }

    #[derive(Debug)]
    pub struct Attr {
        pub name: String,
        pub flags: Vec<Flag>,
        pub vtype: DbPrimitive,
    }

    #[derive(Debug)]
    pub struct Schema {
        id: u32,
        attributes: Vec<Attr>,
    }

    impl Schema {
        pub fn new(attrs: Vec<Attr>) -> Schema {
            Schema {
                id: 0,
                attributes: attrs,
            }
        }

        fn validate(&self, _row: &Row) -> Result<(), String> {
            Ok(())
        }
    }

    pub type Row = Vec<(String, DbPrimitive)>;

    #[derive(Debug)]
    pub struct Table {
        schema: Schema,
        name: String,
        id: u32,
        pub rows: Vec<Row>,
    }

    impl Table {
        pub fn new(name: &str, schema: Schema) -> Table {
            Table {
                id: 1,
                rows: vec![],
                name: name.to_string(),
                schema: schema,
            }
        }

        pub fn insert(&mut self, row: Row) -> Result<String, String> {
            match self.schema.validate(&row) {
                Ok(_) => {
                    // We should make sure that `rows` is not > `isize::MAX bytes`
                    // to prevent it to panic or abort:
                    if self.rows.len() as isize > std::isize::MAX {
                        Err("rows len reached max size".to_string())
                    } else {
                        self.rows.push(row);
                        Ok(String::from("test"))
                    }
                }
                Err(_err) => Err(String::from("Validation error")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::db::*;
    #[test]
    fn create_table() {
        let attributes: Vec<Attr> = vec![Attr {
            name: "name".to_string(),
            flags: vec![],
            vtype: DbPrimitive::String(None),
        }];
        let schema: Schema = Schema::new(attributes);
        let mut table: Table = Table::new("newTable", schema);
        let new_row: Row = vec![(
            "name".to_string(),
            DbPrimitive::String(Some("Bob".to_string())),
        )];
        match table.insert(new_row) {
            Ok(_) => {
                let (k, v): &(String, DbPrimitive) = &table.rows[0][0];
                assert_eq!(k, "name");
                assert_eq!(*v, DbPrimitive::String(Some("Bob".to_string())));
            }
            Err(e) => panic!(e),
        };
    }
    #[test]
    fn validate_valid_row() {}
}
