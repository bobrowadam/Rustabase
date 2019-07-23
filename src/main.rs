enum Flag {
    Unique,
    Required
}

enum Type {
    Int(u64),
    String(String)
}

struct Attr {
    name: String,
    flags: Vec<Flag>,
    vtype: Type
}


struct Schema {
    id: u32,
    attributes: Vec<Attr>
}

impl Schema {
    pub fn new(attrs: Vec<Attr>) -> Schema {
        Schema {
            id: 0,
            attributes: attrs
        }
    }

    fn validate(&self, row: &Row) -> Result<(), String> {
        Ok(())
    }
}

type Row = Vec<(String, Attr)>;

struct Table<'a> {
    schema: Schema,
    name: String,
    id: u32,
    rows: Vec<&'a Row>
}

impl<'a> Table<'a> {
    pub fn new(name: &str, schema: Schema) -> Table {
        Table {
            id: 1,
            rows: vec![],
            name: name.to_string(),
            schema: schema
        }
    }

    pub fn insert(&mut self, row: &'a Row) -> Result<String, String> {
        match self.schema.validate(&row) {
            Ok(_) => {
                self.rows.push(&row); // TODO: handle panics
                Ok(String::from("test"))
            },
            Err(err) => Err(String::from("Validation error"))
        }
    }
}

fn main() {

}
