#![forbid(unsafe_code)]
/*! # Example encode:
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
# Example decode:
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

# Reference:

<https://www.faa.gov/sites/faa.gov/files/air_traffic/technology/adsb/archival/GDL90_Public_ICD_RevA.PDF>
*/

pub mod error;
pub mod extended;
pub mod geometric;
pub mod heartbeat;
pub mod message;
pub mod payload;
pub mod report;
