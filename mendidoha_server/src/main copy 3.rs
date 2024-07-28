use futures::stream::{self, StreamExt};
use tokio::io::{self, AsyncReadExt};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Create a string and convert it to bytes
    let data = "This is a test string that will be streamed.";
    let bytes = data.as_bytes().to_vec();

    // Create a stream from the bytes
    let byte_stream = stream::iter(bytes.into_iter().map(Ok));

    // Read the byte stream asynchronously
    read_byte_stream(byte_stream).await;

    Ok(())
}

async fn read_byte_stream<S>(mut byte_stream: S)
where
    S: futures::stream::Stream<Item = io::Result<u8>> + Unpin,
{
    let mut buffer = Vec::new();

    while let Some(byte_result) = byte_stream.next().await {
        match byte_result {
            Ok(byte) => buffer.push(byte),
            Err(e) => eprintln!("Error reading byte: {}", e),
        }
    }

    // Convert the buffer to a string and print it
    if let Ok(result_string) = String::from_utf8(buffer) {
        println!("Streamed string: {}", result_string);
    } else {
        eprintln!("Failed to convert bytes to string");
    }
}
