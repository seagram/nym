struct Alias {
    name: String,
    category: Option<String>,
    description: Option<String>,
    command: String,
}

fn create_alias(
    name: String, 
    category: impl Into<Option<String>>, 
    description: impl Into<Optional<String>>,
    command: String
    ) -> Alias {
    Alias {
        name,
        category: category.into(),
        description: description.into(),
        command,
    }
}