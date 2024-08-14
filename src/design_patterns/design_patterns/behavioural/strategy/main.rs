use std::collections::HashMap;

type Data = HashMap<String, u32>;

trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}
struct Report;

impl Report {
    fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);

        g.format(&data, s);
    }
}

struct Text;

impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry)
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop();
        }
        buf.push(']');
    }
}



fn main() {

    let mut s = String::from("");
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    let mut s2 = String::from("");
    Report::generate(Json, &mut s2);
    println!("{s2}");
    assert!(s2.contains(r#"{"one":"1"}"#));
    assert!(s2.contains(r#"{"two":"2"}"#));

}