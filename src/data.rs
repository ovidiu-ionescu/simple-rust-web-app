#[derive(Debug, Clone)]
pub struct Data {
    pub id: String,
    pub name: String,
    pub error: String,
}

impl Data {
    pub async fn fetch_data() -> Vec<Data> {
        vec![
            Data {
                id: String::from("id1"),
                name: String::from("Name 1"),
                error: String::from(""),
            },

            Data {
                id: String::from("id2"),
                name: String::from("Name 2"),
                error: String::from("Some error"),
            },
        ]
    }
}
