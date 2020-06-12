use redis::cluster::ClusterClient;
use redis::Commands;

fn fetch_an_integer() -> String {
    // connect to redis
    let nodes = vec!["redis://127.0.0.1:7000/"];
    let client = ClusterClient::open(nodes).unwrap();
    let mut connection = client.get_connection().unwrap();
    let _: () = connection.set("test", "test_data").unwrap();
    let rv: String = connection.get("test").unwrap();
    return rv;
}

fn main() {
    println!("Hello, world!");
    println!("{}", fetch_an_integer())
}
