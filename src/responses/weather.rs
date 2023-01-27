use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    id: u64,
    main: String,
    description: String,
    icon: String
}