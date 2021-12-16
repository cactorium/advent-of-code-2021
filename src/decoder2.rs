#[derive(Debug)]
struct BitView<'a> {
    bytes: &'a [u8],
    bit_offset: usize,
    bit_count: usize,
}

impl <'a> BitView<'a> {
    fn new<'b>(val: &'b [u8]) -> BitView<'b> {
        BitView {
            bytes: val,
            bit_offset: 7,
            bit_count: 0
        }
    }

    fn len(&self) -> usize {
        self.bytes.len() * 8 - (7 - self.bit_offset)
    }

    fn get_bit(&mut self) -> bool {
        let val = (self.bytes[0] & (1 << self.bit_offset)) != 0;
        if self.bit_offset == 0 {
            self.bit_offset = 7;
            self.bytes = &self.bytes[1..self.bytes.len()];
        } else {
            self.bit_offset = self.bit_offset - 1;
        }
        self.bit_count += 1;
        val
    }

    fn get_bits(&mut self, len: usize) -> usize {
        (0..len).fold(0, |acc, _| if self.get_bit() {
            (acc << 1) | 1
        } else {
            acc << 1
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Typ {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo
}

impl Typ {
    fn from_usize(v: usize) -> Option<Typ> {
        match v {
            0 => Some(Typ::Sum),
            1 => Some(Typ::Product),
            2 => Some(Typ::Min),
            3 => Some(Typ::Max),
            4 => Some(Typ::Literal),
            5 => Some(Typ::GreaterThan),
            6 => Some(Typ::LessThan),
            7 => Some(Typ::EqualTo),
            _ => None
        }
    }
}

#[derive(Debug)]
enum Data {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(Vec<u8>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

impl Data {
    fn wrap_operator(typ: Typ, packets: Vec<Packet>) -> Data {
        match typ {
            Typ::Sum => Data::Sum(packets),
            Typ::Product => Data::Product(packets),
            Typ::Min => Data::Min(packets),
            Typ::Max => Data::Max(packets),
            Typ::Literal => unreachable!("shouldn't have a literal type here"),
            Typ::GreaterThan => Data::GreaterThan(packets),
            Typ::LessThan => Data::LessThan(packets),
            Typ::EqualTo => Data::EqualTo(packets),
        }
    }
}


#[derive(Debug)]
struct Packet {
    pub version: usize,
    pub len: usize,
    pub data: Data,
}

impl Packet {
    fn value(&self) -> usize {
        match &self.data {
            &Data::Sum(ref packets) => packets
                .iter()
                .map(|p| p.value())
                .sum(),
            &Data::Product(ref packets) => packets
                .iter()
                .map(|p| p.value())
                .product(),
            &Data::Min(ref packets) => packets
                .iter()
                .map(|p| p.value())
                .min()
                .unwrap(),
            &Data::Max(ref packets) => packets
                .iter()
                .map(|p| p.value())
                .max()
                .unwrap(),
            &Data::Literal(ref val) => val
                .iter()
                .rev()
                .fold(0, |acc, byt| (acc << 8) | (*byt as usize)),
            &Data::GreaterThan(ref packets) => {
                if packets.len() != 2 {
                    panic!("gt packet must have len 2");
                }
                if packets[0].value() > packets[1].value() {
                    1
                } else {
                    0
                }
            },
            &Data::LessThan(ref packets) => {
                if packets.len() != 2 {
                    panic!("lt packet must have len 2");
                }
                if packets[0].value() < packets[1].value() {
                    1
                } else {
                    0
                }
            },
            &Data::EqualTo(ref packets) => {
                if packets.len() != 2 {
                    panic!("eq packet must have len 2");
                }
                if packets[0].value() == packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn decode<'a> (bits: &mut BitView<'a>) -> Option<Packet> {
    println!("decode start {:?}", &bits);
    if bits.len() < 3 {
        println!("ran out of bits, not enough for version!");
        return None;
    }
    let version = bits.get_bits(3);

    if bits.len() < 3 {
        println!("ran out of bits, not enough for type!");
        return None;
    }
    let typ = Typ::from_usize(bits.get_bits(3))?;
    
    println!("v={}, t={:?}", version, typ);
    if typ == Typ::Literal {
        println!("literal packet");
        let mut nibbles: Vec<u8> = Vec::new();
        if bits.len() < 5 {
            println!("invalid packet; ending nibble not found");
            return None;
        }
        let mut nibble = bits.get_bits(5);
        println!("got nibble {}", nibble);

        while (nibble & (1 << 4)) != 0 {
            nibbles.push((nibble & 0x0f) as u8);
            if bits.len() < 5 {
                println!("invalid packet; ending nibble not found");
                return None;
            }
            nibble = bits.get_bits(5);
        }
        println!("got last nibble {}", nibble);
        nibbles.push((nibble & 0x0f) as u8);
        let len = 4*nibbles.len();
        println!("got nibbles {:?}", &nibbles);

        // combine in reverse order so that even/odd nibbles are combined correctly
        let bytes: Vec<u8> = nibbles
            .rchunks(2)
            .map(|chunk| chunk.iter().fold(0u8, |acc, v| (acc << 4) | *v))
            .collect();

        Some(Packet {
            version: version,
            len: len,
            data: Data::Literal(bytes),
        })
    } else {
        println!("operator packet");
        let len_is_num_packets = bits.get_bit();
        if len_is_num_packets {
            let len = bits.get_bits(11);
            println!("np={}", len);
            let mut packets = Vec::new();
            for _ in 0..len {
                let packet = decode(bits)?;
                packets.push(packet);
            }
            let bitlen = packets.iter().map(|p| p.len).sum();
            Some(Packet {
                version: version,
                len: bitlen,
                data: Data::wrap_operator(typ, packets),
            })
        } else {
            let bitlen = bits.get_bits(15);
            println!("len={}", bitlen);
            let mut bits_read = 0;
            let mut packets = Vec::new();
            while bits_read < bitlen {
                let startlen = bits.len();
                let packet = decode(bits)?;
                let endlen = bits.len();
                bits_read += startlen - endlen;
                packets.push(packet);
                println!("total bit count = {}", bits_read);
            }
            if bits_read != bitlen {
                println!("parse error; packet lengths do not add as expected");
                return None;
            }
            Some(Packet {
                version: version,
                len: bitlen,
                data: Data::wrap_operator(typ, packets),
            })
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut input_str = String::new();
    stdin.read_line(&mut input_str).unwrap();

    let input: Vec<u8> = input_str.trim()
        .bytes()
        .map(|b| if b >= ('0' as u8) && b <= ('9' as u8) {
            b - ('0' as u8)
        } else if b >= ('A' as u8) && b <= ('F' as u8) {
            b - ('A' as u8) + 10
        } else if b >= ('a' as u8) && b <= ('f' as u8) {
            b - ('a' as u8) + 10
        } else {
            unreachable!("invalid input, not hex");
        })
        .fold((0, 0, Vec::new()), |(mut num_bits, mut tmp, mut buf), nibble| {
            tmp = (tmp << 4) | nibble;
            num_bits += 4;
            if num_bits == 8 {
                buf.push(tmp);
                num_bits = 0;
                tmp = 0;
            }
            (num_bits, tmp, buf)
        })
        .2;
    let mut view = BitView::new(&input);
    let packet = decode(&mut view).unwrap();
    println!("{:?}", &packet);
    println!("{}", packet.value());
}
