#[derive(Debug)]
enum Flag {
    Unique,
    Required
}

#[derive(Debug)]
enum Type {
    Int(Option<i32>),
    String(Option<String>)
}

#[derive(Debug)]
struct Attr {
    name: String,
    flags: Vec<Flag>,
    vtype: Type
}

#[derive(Debug)]
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
type Row = Vec<(String, Type)>;

#[derive(Debug)]
struct Table {
    schema: Schema,
    name: String,
    id: u32,
    rows: Vec<Row>
}

impl Table {
    pub fn new(name: &str, schema: Schema) -> Table {
        Table {
            id: 1,
            rows: vec![],
            name: name.to_string(),
            schema: schema
        }
    }

    pub fn insert(&mut self, row: Row) -> Result<String, String> {
        match self.schema.validate(&row) {
            Ok(_) => {
                self.rows.push(row); // TODO: handle panics
                Ok(String::from("test"))
            },
            Err(err) => Err(String::from("Validation error"))
        }
    }
}

fn doStuff(table: &mut Table) {
    let row: Row = vec![("id".to_string(), Type::String(Some("Hello".to_string())))];
    table.insert(row);
}

fn main() {
    let attributes: Vec<Attr> = vec![
        Attr{ name: "name".to_string(), flags: vec![], vtype: Type::String(None) }
    ];
    let schema: Schema = Schema::new(attributes);
    let mut table: Table = Table::new("newTable", schema);
    doStuff(&mut table);
    println!("{:?}", table);
}
