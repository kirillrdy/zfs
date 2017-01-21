use std::process::Command;

//extern crate regex;

#[derive(Debug)]
struct Dataset {
    name: String,
    used: String,
    available: String,
    referred: String,
    mountpoint: String
}

fn main() {
    //TODO remove all unwraps
    //TODO name things better
    let output = Command::new("zfs").arg("list").arg("-H").output().unwrap().stdout;
    let output = String::from_utf8(output).unwrap();
    for item in output.split("\n") {

        if item == "" {
            continue
        }

        let values: Vec<&str> = item.split("\t").collect();
        let dataset = Dataset{
            //TODO difference between owned and to_string
            name: values[0].to_string(), //TODO can values[n] crash ?
            used: values[1].to_string(), //TODO can values[n] crash ?
            available: values[2].to_string(), //TODO can values[n] crash ?
            referred: values[3].to_string(), //TODO can values[n] crash ?
            mountpoint: values[4].to_string(), //TODO can values[n] crash ?
        };
        println!("{:?}", dataset);
    }
}
