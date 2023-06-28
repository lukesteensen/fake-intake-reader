use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    payloads: Vec<Payload>,
}

#[derive(Debug, Deserialize)]
struct Payload {
    // timestamp: String,
    data: String,
}

#[derive(Debug, Deserialize)]
struct Foo {
    series: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    points: Vec<(u64, f32)>,
}

/// Script
/// start everything
/// kill fakeintake
/// wait 2 mins
/// restart agent
/// restart fake intake
/// result on main => 149s gap
/// result on branch => 15s gap

fn main() -> Result<()> {
    // keep track of all timestamps on individual points sent to intake
    let mut times = get_times()?;

    loop {
        let gap = times.windows(2).map(|ts| ts[1] - ts[0]).max();
        println!("{} times, largest gap is {:?}", times.len(), gap);

        std::thread::sleep(std::time::Duration::from_secs(5));

        if let Ok(new) = get_times() {
            times.extend_from_slice(new.as_slice());
            times.sort_unstable();
            times.dedup();
        }
    }
}

fn get_times() -> Result<Vec<u64>> {
    let body: Response = reqwest::blocking::get(
        "http://localhost:8080/fakeintake/payloads?endpoint=/api/v1/series",
    )?
    .json()?;

    let mut times = Vec::new();
    for payload in body.payloads {
        let parsed = general_purpose::STANDARD.decode(payload.data)?;
        let mut d = flate2::read::ZlibDecoder::new(parsed.as_slice());
        let foo: Foo = serde_json::from_reader(&mut d)?;
        for serie in foo.series {
            for (t, _) in serie.points {
                times.push(t);
            }
        }
    }
    times.sort_unstable();
    times.dedup();

    Ok(times)
}
