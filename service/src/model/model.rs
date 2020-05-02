#[derive(Debug, Clone)]
pub struct Model<ID, DATA> {
    pub id: ID,
    pub data: DATA,
}
