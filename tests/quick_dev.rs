use anyhow::Result;
use serde_json::json;

/// for fast debugging, we need to emulate custom web client's behaviors.
/// `httpc_test` crate does these things! HORRAY
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Mike").await?.print().await?;
    hc.do_get("/hello?name=Wheatley").await?.print().await?;
    hc.do_get("/hello.txt").await?.print().await?;
    hc.do_get("/unknown/route").await?.print().await?; // this gonna return 404 Not Found

    // test login api
    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    // test wrong login info
    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "nice_to_meet_you",
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
