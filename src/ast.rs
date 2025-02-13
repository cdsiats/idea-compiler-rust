pub struct Schema {
    pub imports: Vec<ImportStatement>,
    pub models: Vec<ModelDeclaration>,
    pub types: Vec<TypeDeclaration>,
    pub enums: Vec<EnumDeclaration>,
    pub plugins: Vec<PluginDefinition>,
    pub props: Vec<PropertyDeclaration>,
}

pub struct ImportStatement {
    pub path: String,
}

pub struct PluginDefinition {
    pub path: String,
    pub attributes: Vec<Property>
}

pub struct ModelDeclaration {
    pub name: String,
    pub columns: Vec<ColumnConfig>
}

pub struct TypeDeclaration {
    pub name: String,
    pub columns: Vec<ColumnConfig>,
}

pub struct EnumDeclaration {
    pub name: String,
    pub columns: Vec<(String, String)>,
}

pub struct PropertyDeclaration {
    pub name: String,
    pub attributes: Vec<Property>
}

pub struct ColumnConfig {
    pub name: String,
    pub data_type: String,
    pub required: bool,
    pub multiple: bool,
    pub attributes: Vec<Property>,
}

pub struct Property {
    pub key: String,
    pub value: Expression,
}

pub enum Expression {
    Identifier(String),
    Literal(Literal),
    List(Vec<Expression>),
    Object(Vec<Property>),
}

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}


