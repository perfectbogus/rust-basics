use std::path::PathBuf;
use std::time::Duration;

#[derive(Default, PartialEq, Debug)]
struct MyConfiguration {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool
}

fn main() {
    let mut conf = MyConfiguration::default();
    conf.check = true;

    println!("conf = {conf:?}");

    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };

    assert_eq!(conf, conf1);

}