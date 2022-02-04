//--- Day 16: Packet Decoder ---

//As you leave the cave and reach open waters, you receive a transmission from the Elves back on the ship.

//The transmission was sent using the Buoyancy Interchange Transmission System (BITS), a method of packing numeric expressions into a binary sequence. Your submarine's computer has saved the transmission in hexadecimal (your puzzle input).

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::utils;

#[derive(Clone)]
struct U32Words(Vec<u32>);


#[derive(Error, Debug, Clone)]
pub enum DecodeError {
    #[error("Invalid command")]
    InvalidError(String),
}


impl FromStr for U32Words {
    type Err = DecodeError;

    // packs the hex digits into a u32 as 8 hex digits.
    // Note that they are left to right, i.e. MSB, and
    // missing digits are still left shifted.
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        line
            .chars()
            .collect::<Vec<_>>()
            .chunks(8)
            .map(|cs|
                cs.iter()
                  .map(|c| (*c).to_digit(16)
                               .ok_or_else(|| DecodeError::InvalidError(
                                       format!("bad digit '{}' in line: {}", c, line))))
                  .collect::<Result<Vec<_>, _>>()
                  // convert 4 u32 'hex' digits (0-15) into a single u32 word.
                  // left justified msb high.
                  .and_then(|cs| {
                    let mut v: u32 = 0;
                    let mut c: usize = 0;
                    for &d in cs.iter() {
                        v *= 16;
                        v += d;
                        c += 1;
                    }
                    while c < 8 {
                        v *= 16;
                        c += 1;
                    }
                    Ok(v)
                  }))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v))
            .map_err(|e| DecodeError::InvalidError(format!("Couldn't parse: {}", e)))
    }
}


impl U32Words {
    fn parse_lines<S>(lines: &[S]) -> Result<Self, DecodeError>
        where S: AsRef<str>
    {
        let vec_u32words = lines
            .iter()
            .map(|l| l.as_ref().parse::<Self>())
            .collect::<Result<Vec<_>, _>>()?;
        let mut out: Vec<u32> = Vec::new();
        for mut u32word in vec_u32words {
            out.append(&mut u32word.0);
        }
        Ok(Self(out))
    }

    fn bit_reader(&self) -> U32WordsReader {
        U32WordsReader::new(self)
    }
}


impl fmt::Display for U32Words {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for word in self.0.iter() {
            write!(f, "{:x?}", word)?;
        }
        Ok(())
    }
}


impl fmt::Debug for U32Words {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U32Words([")?;
        for (i, word) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:x?}", word)?;
        }
        write!(f, "])")
    }
}

// an iterator like structure that reads through the U32Words by a number of bits.
// i.e calling next_bits(n) provides a (m, v) where 'm' is the number of bits read
// and 'v' is the u32 of those bits right shifted.  (e.g. if 3 bits were asked for,
// then the u32 will have those 3 bits in the lowest position.  The struct also has
// read only structures for the length, current position, and remaining bits in the
// reader.  Note that the method takes a reference to the U32Words with a lifetime.
#[derive(Clone, Debug)]
struct U32WordsReader<'a> {
    words: &'a U32Words,
    position: usize,
    length: usize,
    remaining: usize,
}

impl<'a> U32WordsReader<'a> {
    fn new(words: &'a U32Words) -> Self {
        let length = words.0.len() * 32;
        Self {words, position: 0, length, remaining: length}
    }

    fn next_bits(&mut self, num: usize) -> Option<u32> {
        if num == 0 || num > 32 || self.position + num > self.length {
            return None;
        }
        let end_position = self.position + num -1;
        let start_word = self.position / 32;
        let end_word = end_position / 32;
        let end_msb_in_word = 31 - (end_position % 32);   // e.g. how many right shifts required.
        let res = if start_word == end_word {
            // the mask is inside the word and thus we shift right by end_msb_in_word and the
            // mask off the bits of usize.
            (self.words.0[start_word] >> end_msb_in_word) & mask_from_end(num)
        } else {
            // we need all of the 1st word << (32 - end_msb_in_word) and then or-ed with
            // the 2nd word >> end_msb_in_word.
            ((self.words.0[start_word] << (32 - end_msb_in_word)) |
               (self.words.0[end_word] >> end_msb_in_word)) & mask_from_end(num)

        };
        self.position += num;
        self.remaining -= num;
        // start_byte and end_byte will differ by 0 or 1.
        Some(res)
    }
}


// masks for shifts

// number of bits of bask, 0 -> 32 from the LSB.  0 => 0x0000, 1 => 0x0001, 2 => 0x0003, etc.
fn mask_from_end(bits: usize) -> u32 {
    if bits == 0 {
        return 0;
    }
    if bits > 32 {
        panic!("mask_from_end asked for more than 32 bits");
    }
    let mask: u64 = (1 << bits) -1;
    mask as u32
}


// packet decoders and holding packets.

#[derive(Clone, Debug)]
enum Packet {
    // Type ID:4
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        operator: u8,    // this is the id at the moment; so far, anyway
        packets: Vec<Packet>,
    },
}


// decode (recursively as necessary) at the current position into
// a packet.
fn decode(bit_reader: &mut U32WordsReader) -> Result<Packet, String> {
    let version = bit_reader.next_bits(3)
                    .ok_or(String::from("Couldn't get version"))?;
    let type_id = bit_reader.next_bits(3)
                    .ok_or(String::from("Couldn't get type_id"))?;
    if type_id == 4 {
        // it's a literal, let'd try to decode it.
        let mut v: u64 = 0;
        loop {
            let last = bit_reader.next_bits(1)
                    .ok_or(String::from("Couldn't read literal group header"))?;
            let value = bit_reader.next_bits(4)
                    .ok_or(String::from("Couldn't read literal group header"))?;
            v *= 16;
            v += value as u64;
            if last == 0 {
                break;
            }
        }
        return Ok(Packet::Literal {version: version as u8, value: v});
    } else {
        let length_type = bit_reader.next_bits(1)
                        .ok_or(String::from("Couldn't get length type"))?;
        if length_type == 0 {
            // number of bits in the following packets is encoded in next field
            let sub_pkt_length = bit_reader.next_bits(15)
                        .ok_or(String::from("Couldn't get sub-packet length"))?;
            let end_pos = bit_reader.position + (sub_pkt_length as usize);
            let mut sub_pkts: Vec<Packet> = Vec::new();
            while bit_reader.position < end_pos {
                let sub_pkt = decode(bit_reader)?;
                sub_pkts.push(sub_pkt);
            }
            return Ok(Packet::Operator { version: version as u8,
                                         operator: type_id as u8,
                                         packets: sub_pkts});
        } else {
            // number of packets is encoded in next field
            let num_sub_pkts = bit_reader.next_bits(11)
                        .ok_or(String::from("Couldn't get the number of following packets"))?;
            let mut sub_pkts: Vec<Packet> = Vec::new();
            for _ in 0..num_sub_pkts {
                let sub_pkt = decode(bit_reader)?;
                sub_pkts.push(sub_pkt);
            }
            return Ok(Packet::Operator { version: version as u8,
                                         operator: type_id as u8,
                                         packets: sub_pkts});
        }
    }
}


fn part1_sum_version(packet: &Packet) -> u32 {
    match packet {
        Packet::Literal{version, ..} => *version as u32,
        Packet::Operator{version, operator: _, packets} => ((*version as u32) +
            packets.iter().map(|p| part1_sum_version(p)).sum::<u32>()) as u32,
    }
}


pub fn day16_1() {
    println!("Day 16: Packet Decoder, part 1");
    let lines = utils::read_file_single_result::<String>("./input/day16.txt")
        .expect("Couldn't read file");
    //println!("Input: {:?}", &lines);
    let input = U32Words::parse_lines(&lines).expect("Couldn't decode words");
    //println!("Input is: {}", &input);
    let mut bit_reader = input.bit_reader();
    let pkt = decode(&mut bit_reader).expect("Couldn't decode?");
    //println!("pkt: {:?}, bit_reader: {:?}", &pkt, bit_reader);
    println!("part1 version sum: {}", part1_sum_version(&pkt));
}


