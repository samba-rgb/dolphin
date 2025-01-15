pub trait Query {
    fn find(&self) -> String;
}

pub struct SQL;
pub struct Mongo;

impl Query for SQL {
    fn find(&self) -> String {
        "Executing SQL find".to_string()
    }
}

impl Query for Mongo {
    fn find(&self) -> String {
        "Executing Mongo find".to_string()
    }
}

/// Enum representing supported database backends.
pub enum DbBackend {
    Sql,
    Mongo,
}

impl DbBackend {
    /// Returns the string representation of the database backend.
    pub fn as_str(&self) -> &str {
        match self {
            DbBackend::Sql => "SQL",
            DbBackend::Mongo => "Mongo",
        }
    }

    /// Parses a string and returns the corresponding `DbBackend`.
    pub fn from_str(value: &str) -> DbBackend {
        match value {
            "SQL" => DbBackend::Sql,
            "Mongo" => DbBackend::Mongo,
            _ => DbBackend::Sql, // Default to SQL if unmatched
        }
    }
}

/// Returns the appropriate database strategy based on the given `DbBackend`.
pub fn get_db_strategy(backend: DbBackend) -> Box<dyn Query> {
    match backend {
        DbBackend::Sql => Box::new(SQL),
        DbBackend::Mongo => Box::new(Mongo),
    }
}


