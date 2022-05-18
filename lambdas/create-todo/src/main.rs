use aws::{Env, DB};
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_http::{service_fn, Error};
use lambda_runtime::LambdaEvent;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handle)).await?;
    Ok(())
}

async fn handle(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    let db = DB::new().await;
    let env = Env::new();

    let id = Uuid::new_v4().to_string();
    let text = event["text"].as_str().ok_or("text is required")?;

    db.client()
        .put_item()
        .table_name(env.table_todos())
        .item("id", AttributeValue::S(id.to_string()))
        .item("text", AttributeValue::S(text.to_string()))
        .item("completed", AttributeValue::Bool(false))
        .send()
        .await?;

    Ok(json!({
        "id": id,
        "text": text,
        "completed": false
    }))
}
