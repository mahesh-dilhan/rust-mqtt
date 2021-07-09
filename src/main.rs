use mqttrs::*;
use bytes::BytesMut;

// Allocate write buffer.
let mut buf = BytesMut::with_capacity(1024);

// Encode an MQTT Connect packet.
let pkt = Packet::Connect(Connect { protocol: Protocol::MQTT311,
keep_alive: 30,
client_id: "doc_client".into(),
clean_session: true,
last_will: None,
username: None,
password: None });
assert!(encode(&pkt, &mut buf).is_ok());
assert_eq!(&buf[14..], "doc_client".as_bytes());
let mut encoded = buf.clone();

// Decode one packet. The buffer will advance to the next packet.
assert_eq!(Ok(Some(pkt)), decode(&mut buf));

// Example decode failures.
let mut incomplete = encoded.split_to(10);
assert_eq!(Ok(None), decode(&mut incomplete));
let mut garbage = BytesMut::from(&[0u8,0,0,0] as &[u8]);
assert_eq!(Err(Error::InvalidHeader), decode(&mut garbage));
