use crate::{
    byte_packet_buffer::BytePacketBuffer, 
    query_type::QueryType
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion {
            name: name,
            qtype: qtype
        }
    }
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(),String> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        let _ = buffer.read_u16()?;
        Ok(())
    }
    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), String>{
        buffer.write_qname(&self.name)?;
        let typenum = self.qtype.to_num();
        buffer.write_u16(typenum)?;
        buffer.write_u16(1)?;
        Ok(())
    }
}