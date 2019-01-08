use select::document::Document;
use select::predicate::Name;

fn main() {
    schedule("https://news.ycombinator.com");
    //schedule("https://web.skola24.se/timetable/timetable-viewer/kunskapsforbundetvast.skola24.se/Kunskapsf%C3%B6rbundet%20Gy/signatures/1196/");
}

fn schedule(url: &str) {
    println!("Started");
    let resp = reqwest::get(url);

    match resp {
        Ok(res) => {
            println!("Request finished. ");

            Document::from_read(res)
                .unwrap()
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| println!("{}", x));

            println!("Finished")
        },
        Err(x) => println!("Failed: {:?}", x)
    }
}

/*use std::fs::File;
use std::io::prelude::*;

fn main() {
    let client = reqwest::Client::new();

    let mut first = String::from("");
    let mut second = String::from("");

    for _ in 0..5 {
        first = client.get("https://web.skola24.se/timetable/timetable-viewer/kunskapsforbundetvast.skola24.se/Kunskapsf%C3%B6rbundet%20Gy/signatures/1196/").send().unwrap().text().unwrap();
        second = client.get("https://web.skola24.se/timetable/timetable-viewer/kunskapsforbundetvast.skola24.se/Kunskapsf%C3%B6rbundet%20Gy/signatures/1196/").send().unwrap().text().unwrap();
    }

    let mut file = File::create("result.html").unwrap_or_else(|err| {
        panic!("Could not create file. Error: {}", err);
    });

    let output = first + &second;

    let _ = file.write_all(output[..].as_bytes()).unwrap();
}
*/
/*
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", "curl localhost/bah.php"])
                .output()
                .expect("Failed to execute process. ")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg("curl localhost/bah.php")
        .output()
        .expect("Failed to execute process. ")
    };

    let mut file = File::create("result.html").unwrap_or_else(|err| {
        panic!("Could not create file. Error: {}", err);
    });

    let _ = file.write_all(&output.stdout).unwrap();
}
*/
