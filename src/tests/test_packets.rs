#[test]
fn test_load_response_packet() {
    use crate::byte_packet_buffer::BytePacketBuffer;
    use crate::dns_packet::DnsPacket;
    use std::fs;
    use std::io::Read;
    let mut f = fs::File::open("response_packet.txt").unwrap();
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf).unwrap();
    let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }

    for rec in packet.answers {
        println!("{:#?}", rec);
    }

    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
}