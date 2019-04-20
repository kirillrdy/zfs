mod datasets;
mod types;

fn main() {
    let datasets = datasets::list();
    for dataset in datasets {
        println!("{}", dataset.name)
    }
}
