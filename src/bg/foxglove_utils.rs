use foxglove_ws;
use std::{io::Write, time::SystemTime};

fn build_string_message(data: &str) -> anyhow::Result<Vec<u8>> {
    let mut msg = vec![0; std::mem::size_of::<u32>() + data.len()];
    // ROS 1 message strings are encoded as 4-bytes length and then the byte data.
    let mut w = std::io::Cursor::new(&mut msg);
    w.write(&(data.len() as u32).to_le_bytes())?;
    w.write(data.as_bytes())?;
    Ok(msg)
}

#[tokio::main]
pub async fn test() -> anyhow::Result<()> {
    // Init the struct
    let server = foxglove_ws::FoxgloveWebSocket::new();

    // Open the server connection
    tokio::spawn({
        let server = server.clone();
        async move { server.serve(([127, 0, 0, 1], 8765)).await }
    });

    // Start a channel to talk thru
    let channel = server
        .publish(
            "/hello_up".to_string(),
            "protobuf".to_string(),
            "/hello_up".to_string(),
            "protobuf".to_string(),
            "protobuf".to_string(),
            false,
        )
        .await?;

    // Make a latching channel so that foxglove remebers rare messages better
    // let channel_latching = server
    //     .publish(
    //         "/hello_rare".to_string(),
    //         "protobuf".to_string(),
    //         "/hello_rare".to_string(),
    //         "protobuf".to_string(),
    //         "protobuf".to_string(),
    //         true,
    //     )
    //     .await?;

    // Send an example message
    let mut counter = 0;
    loop {
        channel
            .send(
                SystemTime::now().elapsed().unwrap().as_nanos() as u64,
                &build_string_message(&format!("Hello {}!", counter))?,
            )
            .await?;
        counter += 1;
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    }
}
