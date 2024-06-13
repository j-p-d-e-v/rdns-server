



#[ignore]
#[test]
fn test_load_response_packet() {
    use crate::byte_packet_buffer::BytePacketBuffer;
    use crate::dns_packet::DnsPacket;
    use std::fs;
    use std::io::Read;
    let mut f = fs::File::open("tests/response_packet.txt").unwrap();
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

#[test]
fn test_dns_stub_resolver(){
    use crate::query_type::QueryType;
    use std::net::UdpSocket;
    use crate::{
        byte_packet_buffer::BytePacketBuffer, 
        dns_packet::DnsPacket, 
        dns_question::DnsQuestion
    };

    let qname = "google.com";
    let qtype = QueryType::A;
    let server = ("8.8.8.8",53);

    let socket = UdpSocket::bind(("0.0.0.0",43210)).unwrap();
    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.questions.push(
        DnsQuestion::new(qname.to_string(), qtype)
    );

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();
    
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    let res_packet = DnsPacket::from_buffer(&mut res_buffer).unwrap();
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}",q);
    }
    for rec in res_packet.answers {
        println!("{:#?}",rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}",rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}",rec);
    }
}