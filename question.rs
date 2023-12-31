use crate::message_buffer::MessageBuffer;
use crate::enums::{Type, Class};

#[derive(Debug, Default, Clone)]
pub struct Question {
    pub qname: String, //domain name
    pub qtype: Type, //type of query
    pub qclass: Class //class of query
}

impl From<&mut MessageBuffer> for Question {
    fn from(message: &mut MessageBuffer) -> Question {
        let mut question = Question::default();
        let mut byte = message.next().unwrap_or_default();

        while byte != 0 {
            let qname_count = byte;

            for _ in 0..qname_count {
                let character = message.next().unwrap_or_default() as char;
                question.qname.push(character);
            }

            question.qname.push('.');
            byte = message.next().unwrap_or_default();
        }

        question.qname.pop();

        let qtype = message.next_u16().unwrap_or_default();
        question.qtype = Type::from(qtype);

        let qclass = message.next_u16().unwrap_or_default();
        question.qclass = Class::from(qclass);

        return question;
    }
}

impl Question {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        for word in self.qname.split('.') {
            bytes.push(word.len() as u8);
            bytes.append(&mut word.as_bytes().to_vec());
        }
        
        bytes.push(0);

        bytes.append(&mut <[u8; 2]>::from(self.qtype).to_vec());
        bytes.append(&mut <[u8; 2]>::from(self.qclass).to_vec());

        return bytes;
    }
}
