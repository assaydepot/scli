use scientist_api_sdk::client::sci_get;
use scientist_api_sdk::response::SciInfo;

#[tokio::main]
pub async fn main() {
    let url = "https://app-staging.scientist.com/api/v1/info.json";
    let token = "axveH-GzuLws2D5m1MYV";

    // ::<> ::<> ::<> ::<>

    let result = sci_get(url, token)
        .await
        .unwrap()
        .json::<SciInfo>()
        .await
        .unwrap();
    println!("{:?}", result);
}
