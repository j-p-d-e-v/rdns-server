use crate::byte_packet_buffer::BytePacketBuffer;
use crate::query_type::QueryType;
use crate::dns_packet::DnsPacket;
use crate::dns_question::DnsQuestion;
use std::net::{Ipv4Addr, UdpSocket};
use crate::result_code::ResultCode;

#[derive(Debug)]
pub struct Server;

impl Server {
    pub fn recursive_lookup(qname: &str, qtype: QueryType) -> Result<DnsPacket, String> {
        let mut ns = "198.41.0.4".parse::<Ipv4Addr>().unwrap();

        loop {
            println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);

            let ns_copy = ns;

            let server = (ns_copy, 53);
            let response = Server::lookup(qname, qtype, server)?;

            if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
                return Ok(response);
            }

            if response.header.rescode == ResultCode::NXDOMAIN {
                return Ok(response);
            }

            if let Some(new_ns) = response.get_resolved_ns(qname) {
                ns = new_ns;
                continue;
            }

            let new_ns_name = match response.get_unresolved_ns(qname) {
                Some(x) => x,
                None => return Ok(response)
            };

            let recursive_response = Server::recursive_lookup(&new_ns_name, QueryType::A)?;
            if let Some(new_ns) = recursive_response.get_random_a() {
                ns = new_ns;
            }
            else {
                return Ok(response);
            }
        }
    }

    pub fn lookup(qname: &str, qtype: QueryType, server: (Ipv4Addr, u16)) ->  Result<DnsPacket,String> {
        //let server = ("8.8.8.8",53);
        match UdpSocket::bind(("0.0.0.0",43210)) {
            Ok(socket) => {

                let mut packet = DnsPacket::new();

                packet.header.id = 6666;
                packet.header.questions = 1;
                packet.header.recursion_desired = true;
                packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));
        
                let mut req_buffer = BytePacketBuffer::new();
                packet.write(&mut req_buffer)?;
                if let Err(error) = socket.send_to(&req_buffer.buf[0..req_buffer.pos], server) {
                    return Err(format!("Error (socket:send_to): {:?}",error));
                }
        
                let mut res_buffer = BytePacketBuffer::new();
                if let Err(error) = socket.recv_from(&mut res_buffer.buf) {
                    return Err(format!("Error (socket:recv_from): {:?}",error));
                }
                DnsPacket::from_buffer(&mut res_buffer)
            }
            Err(error) => Err(format!("{:?}",error))
        }
    }

    pub fn handle_query(socket: &UdpSocket) -> Result<(),String> {
        let mut req_buffer = BytePacketBuffer::new();
        
        match socket.recv_from(&mut req_buffer.buf) {
            Ok((_, src)) => {        
                let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

                let mut packet = DnsPacket::new();
                packet.header.id = request.header.id;
                packet.header.recursion_desired = true;
                packet.header.recursion_available = true;
                packet.header.response = true;
        
                if let Some(question) = request.questions.pop() {
                    println!("Received query: {:?}", question);
        
                    if let Ok(result) = Server::recursive_lookup(&question.name, question.qtype) {
                        packet.questions.push(question);
                        packet.header.rescode = result.header.rescode;
        
                        for rec in result.answers {
                            println!("Answer: {:?}", rec);
                            packet.answers.push(rec);
                        }
                        for rec in result.authorities {
                            println!("Authority: {:?}", rec);
                            packet.authorities.push(rec);                    
                        }
                        for rec in result.resources {
                            println!("Resource: {:?}", rec);
                            packet.resources.push(rec);                    
                        }
                    }
                    else {
                        packet.header.rescode = ResultCode::SERVFAIL;
                    }
                }  
                else{
                    packet.header.rescode = ResultCode::FORMERR;
                }      
                let mut res_buffer = BytePacketBuffer::new();
                packet.write(&mut res_buffer)?;
        
                let len = res_buffer.pos();
                let data = res_buffer.get_range(0, len)?;
                if let Err(error) = socket.send_to(data, src) {
                    return Err(format!("{:?}",error));
                }
                Ok(())
            }
            Err(error) => return {
                Err(format!("{:?}",error))
            }
        }
    }
    pub fn launch() {
        let socket = UdpSocket::bind(("0.0.0.0",2053)).unwrap();

        loop {
            match Server::handle_query(&socket) {
                Ok(_) => {},
                Err(error) => eprintln!("An error occured: {}",error)
            }
        }
    }
}