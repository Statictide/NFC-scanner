#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Maybe<T> {
    pub value: Option<T>,
}
