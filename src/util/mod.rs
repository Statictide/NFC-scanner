
#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Maybe<T> {
    pub value: Option<T>,
}