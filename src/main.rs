use std::collections::HashMap;

fn main() {
    let url = "redis://sentinel.service.consul:26379";
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();

    let groups : Vec<HashMap<String, String>> = redis::cmd("SENTINEL").arg("MASTERS").query(&con).unwrap();
    for group in groups {
        let name = &group["name"];
        println!("group {}", name);

        let master : HashMap<String, String> = redis::cmd("SENTINEL").arg("MASTER").arg(name).query(&con).unwrap();
        println!("  master {}:{}", master["ip"], master["port"]);

        show_server_info(&format!("redis://{}:{}", master["ip"], master["port"]));

        let slaves : Vec<HashMap<String, String>> = redis::cmd("SENTINEL").arg("SLAVES").arg(name).query(&con).unwrap();
        for slave in slaves {
            println!("  slave {}:{}", slave["ip"], slave["port"]);
            show_server_info(&format!("redis://{}:{}", slave["ip"], slave["port"]));
        }
    }
}

fn show_server_info(url : &str) {
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();

    let info : String = redis::cmd("INFO").arg("server").query(&con).unwrap();
    for line in info.lines() {
        if line.starts_with("uptime_in_days:") {
            println!("    {}", line);
        }
    }
}
