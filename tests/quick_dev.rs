use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello2/Mike").await?.print().await?;
    hc.do_get("/hello?name=Wheatley").await?.print().await?;
    hc.do_get("/hello.txt").await?.print().await?;
    hc.do_get("/unknown/route").await?.print().await?; // this gonna return 404 Not Found

    Ok(())
}
