mod datasets;

fn main() {
    for dataset in datasets::list().unwrap() {
        println!("{:?}", dataset)
    }
}
