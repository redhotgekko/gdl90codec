![Rust workflow](https://github.com/redhotgekko/gdl90codec/actions/workflows/rust.yml/badge.svg)

*Note: This project is still under construction!*

# GDL90 Codec

This is a partial implementation of the [FAA GDL90 Specification](https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF).

The intent is for this project to eventually be able to encode and decode a reasonably full set of the possible messages.

## Dependency

If you would like to try this project, you can do so by adding it to your Cargo.toml file.

_Note that this project is under development, so the dependency below will change without notice_

```
[dependencies]
gdl90codec = { git = "https://github.com/redhotgekko/gdl90codec.git" }
```

## Example encode:
```rust
use gdl90codec::heartbeat::HeartBeat;
use gdl90codec::message::create_message;
use gdl90codec::payload::Payload;

let mut heartbeat = HeartBeat::default();

heartbeat.gps_pos_valid = true;
heartbeat.uat_initialized = true;
heartbeat.utc_ok = false;

let seconds_since_midnight = 51876; // e.g. chrono::Utc::now().num_seconds_from_midnight();
heartbeat.set_time_stamp(seconds_since_midnight);

let payload = Payload::HeartBeat(heartbeat);

let heartbeat_message = create_message(&payload);

let message_data = heartbeat_message.unwrap().encode();
```
## Example decode:
```rust
use gdl90codec::message::read_message;
use gdl90codec::payload::Payload;

let data = [0x7E, 0x00, 0x81, 0x41, 0xDB, 0xD0, 0x08, 0x02, 0xB3, 0x8B, 0x7E];
let message = read_message(&data[..]).unwrap();

if let Ok(Payload::HeartBeat(payload)) = message.get_payload() {
    println!("{:?}", &payload);
} else {
    panic!("Unexpected payload type");
}
```

# Right to delete

I reserve the right to delete or make private this repository, and related repositories, at my own discretion without notice.

## Development

Before submitting a pull request, please:

Run ```cargo clippy``` and resolve all issues.

Run ```cargo test``` and resolve all issues.

Finally run ```cargo fmt```

## License

Licensed under:

Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
