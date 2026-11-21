
async fn great() -> String {
    String::from("Hello")
}



async fn run() {
    let msg = great().await;
    println!("{msg}");
}


fn main () {
    trpl::block_on(async {
        run().await;
    });
}