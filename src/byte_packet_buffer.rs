#[derive(Debug)]
pub struct BytePacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {

    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buf: [0; 512],
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn step(&mut self, steps: usize) {
        self.pos += steps;
    }

    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn read(&mut self) -> Result<u8,String> {
        if self.pos >= 512 {
            return Err("BytePacketBuffer read: End of buffer".into());
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    pub fn get(&mut self, pos: usize) -> Result<u8,String> {
        if pos >= 512 {
            return Err("BytePacketBuffer get: End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    pub fn get_range(&mut self, start: usize, len: usize) -> Result<&[u8],String> {
        if start + len >= 512 {
            return Err("BytePacketBuffer get_range: End of buffer".into());
        }
        Ok(&self.buf[start..start + len as usize])
    }

    pub fn read_u16(&mut self) -> Result<u16,String> {
        let res: u16 = ( (self.read()? as u16)  << 8)  | (self.read()? as u16);
        Ok(res)
    }

    pub fn read_u32(&mut self) -> Result<u32,String> {
        let res = (self.read()? as u32) << 24 | (self.read()? as u32) << 16 | (self.read()? as u32) << 8 | (self.read()? as u32) << 0;
        Ok(res)
    }

    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(),String> {
        let mut pos = self.pos();
        let mut jumped  = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;
        let mut delim = "";
        loop {
            if jumps_performed > max_jumps {
                return Err(format!("BytePacketBuffer read_qname: Limit of {} jumps exceed", max_jumps));
            }

            let len = self.get(pos)?;

            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2);
                }
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xc0) << 8) | b2;
                pos = offset as usize;
                jumped = true;
                jumps_performed += 1;
                continue;
            }
            else {
                pos += 1;
                if len == 0 {
                    break;
                }
                outstr.push_str(delim);
                let str_buffer = self.get_range(pos, len as usize)?;
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delim = ".";
                pos += len as usize;
            }
        }
        if !jumped {
            self.seek(pos);
        }
        Ok(())
    }    

    pub fn write(&mut self, val: u8) -> Result<(), String>{
        if self.pos >= 512 {
            return Err("BytePacketBuffer write: Error of buffer".to_string());
        }
        self.buf[self.pos] = val;
        self.pos += 1;    
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<(), String>{
        self.write(val)
    }
    
    pub fn write_u16(&mut self, val: u16) -> Result<(), String>{
        self.write(( val >> 8) as u8 )?;
        self.write(( val & 0xFF) as u8 )?;
        Ok(())
    }
    
    pub fn write_u32(&mut self, val: u32) -> Result<(), String>{
        self.write(((val >> 24) & 0xFF) as u8)?;
        self.write(((val >> 16) & 0xFF) as u8)?;
        self.write(((val >> 8) & 0xFF) as u8)?;
        self.write(((val >> 0) & 0xFF) as u8)?;
        Ok(())
    }
    
    pub fn write_qname(&mut self, qname: &str) -> Result<(),String> {
        for label in qname.split(".") {
            let len = label.len();
            if len > 0x3f {
                return Err("BytePacketBuffer write_qname: Error of buffer".to_string());
            }
            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }
        self.write_u8(0)?;
        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) {
        self.buf[pos] = val;
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) {
        self.set(pos, (val >> 8 ) as u8 );
        self.set(pos + 1, (val & 0xFF) as u8);
    }
}
