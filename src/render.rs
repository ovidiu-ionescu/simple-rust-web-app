use askama::Template;

use crate::data::Data;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    data: &'a [Data],
}

pub fn render(data: &[Data]) -> Result<String, askama::Error> {
    let data_template = IndexTemplate { data };
    data_template.render()
}

#[test]
fn test_render() {
    let data = vec![Data { id: String::from("id 1"), name: String::from("Name 1"), error: String::new() }, Data {
        id:    String::from("id 2"),
        name:  String::new(),
        error: String::from("An error"),
    }];
    println!("{}", render(&data).unwrap());
}
