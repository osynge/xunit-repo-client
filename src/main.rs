use glob::glob;
use xunit_repo_interface;
mod into;

fn main() {
    for entry in glob("xml/*.xml").expect("Failed to read glob pattern") {
        let f = match entry {
            Ok(path) => {
                println!("{:?}", path);
                into::try_into(path)
            },
            Err(e) => {println!("{:?}", e); continue;},
        };
        match f {
            Ok(p) => {println!("{:?}", p)},
            Err(p) => {println!("{:?}", p)},
        };

    }
}


fn ssmain() {
    for entry in glob("/home/oms101/home/programming/github/xunit-repo-client/xml/*").expect("Failed to read glob pattern") {
        let f = match entry {
            Ok(path) => {
                into::try_into(path)
            },
            Err(e) => {println!("{:?}", e); continue;},
        };
        match f {Ok(p) => {println!("{:?}", p)},
    Err(_) => {}}
    }
}
