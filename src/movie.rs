use serde::Serialize;


#[derive(Serialize, Debug, Clone)]
pub struct Movie {
    pub id: String,
    pub name: String,
    pub year: u16,
    pub was_good: bool
}