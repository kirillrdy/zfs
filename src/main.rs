mod types;
mod datasets;

fn main() {
    let datasets = datasets::list();
    for dataset in datasets {
        println!("{}", dataset.name)
    }
}
