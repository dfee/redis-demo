use mini_redis::client;

#[tokio::main]
async fn main() {
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();
    let res1 = client.set("foo", "bar".into()).await.unwrap();
    let res2 = client.get("foo").await.unwrap();

    println!("res1={:?} res2={:?}", res1, res2)
}
