use client::error::Error;
use rodio::{Decoder, OutputStream};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // cargo run --example stream D7KyD
    let track_id = std::env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .pop()
        .unwrap_or_else(|| "D7KyD".to_string());

    println!("downloading track {}", track_id);

    let client = client::client::Client::new().build().await?;
    let track_bytes = client.stream_track(&track_id, None).await?;
    let cursor = std::io::Cursor::new(track_bytes);

    println!("downloaded track, decoding");

    let (_stream, stream_handle) =
        OutputStream::try_default().expect("could not get default device");
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let reader = std::io::BufReader::new(cursor);
    let source = Decoder::new(reader).expect("could not decode track");

    println!("playing, press ctrl + c to stop playback");

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
