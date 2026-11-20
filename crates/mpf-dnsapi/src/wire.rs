//! DNS message parsing from Wine's `dlls/dnsapi/record.c`:
//! `DnsExtractRecordsFromMessage_UTF8` and the helpers it uses to walk the
//! sections, decompress names and decode each record's rdata.
//!
//! The message is the raw bytes of a packet. A name is its labels joined by
//! dots, and the root name is `.`.

use crate::{until_nul, Error};

/// `sizeof(DNS_MESSAGE_BUFFER)`: the 12-byte header plus one body byte.
const MIN_MESSAGE_LEN: usize = 13;
/// Wine takes the message length as a `WORD`.
const MAX_MESSAGE_LEN: usize = u16::MAX as usize;
/// `DNS_MAX_NAME_BUFFER_LENGTH`, which counts the terminating NUL.
const MAX_NAME_LEN: usize = 256;

const TYPE_A: u16 = 1;
const TYPE_NS: u16 = 2;
const TYPE_MD: u16 = 3;
const TYPE_MF: u16 = 4;
const TYPE_CNAME: u16 = 5;
const TYPE_SOA: u16 = 6;
const TYPE_MB: u16 = 7;
const TYPE_MG: u16 = 8;
const TYPE_MR: u16 = 9;
const TYPE_NULL: u16 = 10;
const TYPE_PTR: u16 = 12;
const TYPE_HINFO: u16 = 13;
const TYPE_MINFO: u16 = 14;
const TYPE_MX: u16 = 15;
const TYPE_TEXT: u16 = 16;
const TYPE_RP: u16 = 17;
const TYPE_AFSDB: u16 = 18;
const TYPE_X25: u16 = 19;
const TYPE_ISDN: u16 = 20;
const TYPE_RT: u16 = 21;
const TYPE_SIG: u16 = 24;
const TYPE_KEY: u16 = 25;
const TYPE_AAAA: u16 = 28;
const TYPE_SRV: u16 = 33;
const TYPE_OPT: u16 = 41;

/// The section a record came from, with Wine's `DNS_SECTION` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// `DnsSectionAnswer`.
    Answer = 1,
    /// `DnsSectionAddtional`, which Wine spells that way.
    Additional = 3,
}

/// One record from the answer or additional section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: Vec<u8>,
    pub section: Section,
    pub rtype: u16,
    pub ttl: u32,
    pub data: RData,
}

/// The decoded rdata of a record. Each variant holds the fields that Wine's
/// `DNS_*_DATA` union holds for its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RData {
    /// `A`: an IPv4 address, in network order.
    A([u8; 4]),
    /// `AAAA`: an IPv6 address, in network order.
    Aaaa([u8; 16]),
    /// `KEY`: the flags, protocol, algorithm and public key.
    Key {
        flags: u16,
        protocol: u8,
        algorithm: u8,
        key: Vec<u8>,
    },
    /// `MINFO` and `RP`: a mailbox and an errors mailbox.
    Minfo {
        mailbox: Vec<u8>,
        errors_mailbox: Vec<u8>,
    },
    /// `MX`, `AFSDB` and `RT`: a preference and an exchange name.
    Mx { preference: u16, exchange: Vec<u8> },
    /// `CNAME`, `NS`, `MB`, `MD`, `MF`, `MG`, `MR` and `PTR`: one host name.
    Name(Vec<u8>),
    /// `NULL`: the raw rdata.
    Null(Vec<u8>),
    /// `OPT`: the raw rdata.
    Opt(Vec<u8>),
    /// `SIG`: the fixed fields, the signer name and the signature bytes.
    Sig {
        type_covered: u16,
        algorithm: u8,
        labels: u8,
        original_ttl: u32,
        expiration: u32,
        time_signed: u32,
        key_tag: u16,
        signer: Vec<u8>,
        signature: Vec<u8>,
    },
    /// `SOA`: the two names and the five 32-bit fields.
    Soa {
        primary: Vec<u8>,
        admin: Vec<u8>,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum: u32,
    },
    /// `SRV`: the priority, weight, port and target name.
    Srv {
        priority: u16,
        weight: u16,
        port: u16,
        target: Vec<u8>,
    },
    /// `HINFO`, `ISDN`, `TEXT` and `X25`: the character strings in order.
    Strings(Vec<Vec<u8>>),
}

/// `DnsExtractRecordsFromMessage_UTF8`: parses a whole message and returns its
/// answer and additional records in order. Question and authority records are
/// stepped over, and only checked for length.
///
/// The header counts are read little-endian, as Wine reads them on its hosts.
pub fn extract_records(msg: &[u8]) -> Result<Vec<Record>, Error> {
    if msg.len() < MIN_MESSAGE_LEN || msg.len() > MAX_MESSAGE_LEN {
        return Err(Error::InvalidParameter);
    }
    let questions = le16(msg, 4);
    let answers = le16(msg, 6);
    let authorities = le16(msg, 8);
    let additionals = le16(msg, 10);
    if msg[2] & 0x80 != 0 && answers == 0 {
        return Err(Error::BadPacket);
    }

    let mut pos = 12;
    for _ in 0..questions {
        pos = skip_record(msg, pos, true).ok_or(Error::BadPacket)?;
    }
    let mut records = Vec::new();
    for _ in 0..answers {
        let (record, next) = extract_record(msg, pos, Section::Answer)?;
        records.push(record);
        pos = next;
    }
    for _ in 0..authorities {
        pos = skip_record(msg, pos, false).ok_or(Error::BadPacket)?;
    }
    for _ in 0..additionals {
        let (record, next) = extract_record(msg, pos, Section::Additional)?;
        records.push(record);
        pos = next;
    }
    if pos != msg.len() {
        return Err(Error::BadPacket);
    }
    Ok(records)
}

fn le16(msg: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([msg[at], msg[at + 1]])
}

fn be16(msg: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([msg[at], msg[at + 1]])
}

fn be32(msg: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([msg[at], msg[at + 1], msg[at + 2], msg[at + 3]])
}

fn bytes<const N: usize>(msg: &[u8], at: usize) -> [u8; N] {
    msg[at..at + N]
        .try_into()
        .expect("length checked by ensure")
}

/// Fails when the `len` bytes at `at` run past the end of the rdata.
fn ensure(at: usize, len: usize, rrend: usize) -> Result<(), Error> {
    if at + len > rrend {
        return Err(Error::BadPacket);
    }
    Ok(())
}

fn name_at(msg: &[u8], start: usize) -> Result<(Vec<u8>, usize), Error> {
    get_name(msg, start).ok_or(Error::BadPacket)
}

/// `get_name`: reads the name at `start`, following compression pointers. It
/// returns the name and the position where the record goes on, which is the
/// byte after the first pointer if there was one.
fn get_name(msg: &[u8], start: usize) -> Option<(Vec<u8>, usize)> {
    let end = msg.len();
    let mut name = Vec::new();
    let mut next = None;
    let mut ptr = start;
    let mut hops = 0;
    while ptr < end {
        let len = msg[ptr];
        ptr += 1;
        if len == 0 {
            break;
        }
        match len & 0xc0 {
            0 => {
                let n = usize::from(len);
                if name.len() + n + 1 >= MAX_NAME_LEN {
                    return None;
                }
                if !name.is_empty() {
                    name.push(b'.');
                }
                name.extend_from_slice(msg.get(ptr..ptr + n)?);
                ptr += n;
            }
            0xc0 => {
                next.get_or_insert(ptr + 1);
                hops += 1;
                if hops >= end {
                    return None;
                }
                if ptr < end {
                    ptr = (usize::from(len & 0x3f) << 8) + usize::from(msg[ptr]);
                }
            }
            _ => return None,
        }
    }
    if ptr > end {
        return None;
    }
    if name.is_empty() {
        name.push(b'.');
    }
    Some((until_nul(&name).to_vec(), next.unwrap_or(ptr)))
}

/// `skip_name`: steps over the name at `start` without following a pointer.
/// It returns the position after the name, or `None` when that position is not
/// before the end of the message.
fn skip_name(msg: &[u8], start: usize) -> Option<usize> {
    let end = msg.len();
    let mut ptr = start;
    while ptr < end {
        let len = msg[ptr];
        ptr += 1;
        if len == 0 {
            break;
        }
        match len & 0xc0 {
            0 => ptr += usize::from(len),
            0xc0 => {
                ptr += 1;
                break;
            }
            _ => return None,
        }
    }
    (ptr < end).then_some(ptr)
}

/// `skip_record`: steps over a question, or over an authority record with its
/// rdata, and returns the position after it.
fn skip_record(msg: &[u8], start: usize, question: bool) -> Option<usize> {
    let mut ptr = skip_name(msg, start)? + 4;
    if !question {
        ptr += 4;
        if ptr + 2 > msg.len() {
            return None;
        }
        ptr += 2 + usize::from(be16(msg, ptr));
    }
    (ptr <= msg.len()).then_some(ptr)
}

/// `extract_record`: decodes the answer or additional record at `start`.
/// It returns the record and the position after its rdata.
fn extract_record(msg: &[u8], start: usize, section: Section) -> Result<(Record, usize), Error> {
    let (name, mut pos) = name_at(msg, start)?;
    if pos + 10 > msg.len() {
        return Err(Error::BadPacket);
    }
    let rtype = be16(msg, pos);
    let ttl = be32(msg, pos + 4);
    let len = usize::from(be16(msg, pos + 8));
    pos += 10;
    if pos + len > msg.len() {
        return Err(Error::BadPacket);
    }
    let data = rdata(msg, pos, len, rtype)?;
    let record = Record {
        name,
        section,
        rtype,
        ttl,
        data,
    };
    Ok((record, pos + len))
}

/// `extract_rdata`: decodes the `len` bytes of rdata at `pos`. Names in the
/// rdata are read from the whole message, as Wine reads them. Only the SIG
/// signer name must end within the rdata.
fn rdata(msg: &[u8], pos: usize, len: usize, rtype: u16) -> Result<RData, Error> {
    let rrend = pos + len;
    match rtype {
        TYPE_A => {
            ensure(pos, 4, rrend)?;
            Ok(RData::A(bytes(msg, pos)))
        }
        TYPE_AAAA => {
            ensure(pos, 16, rrend)?;
            Ok(RData::Aaaa(bytes(msg, pos)))
        }
        TYPE_KEY => {
            ensure(pos, 4, rrend)?;
            Ok(RData::Key {
                flags: be16(msg, pos),
                protocol: msg[pos + 2],
                algorithm: msg[pos + 3],
                key: msg[pos + 4..rrend].to_vec(),
            })
        }
        TYPE_RP | TYPE_MINFO => {
            let (mailbox, next) = name_at(msg, pos)?;
            let (errors_mailbox, _) = name_at(msg, next)?;
            Ok(RData::Minfo {
                mailbox,
                errors_mailbox,
            })
        }
        TYPE_AFSDB | TYPE_RT | TYPE_MX => {
            ensure(pos, 2, rrend)?;
            let (exchange, _) = name_at(msg, pos + 2)?;
            Ok(RData::Mx {
                preference: be16(msg, pos),
                exchange,
            })
        }
        TYPE_NULL => Ok(RData::Null(msg[pos..rrend].to_vec())),
        TYPE_OPT => Ok(RData::Opt(msg[pos..rrend].to_vec())),
        TYPE_CNAME | TYPE_NS | TYPE_MB | TYPE_MD | TYPE_MF | TYPE_MG | TYPE_MR | TYPE_PTR => {
            let (host, _) = name_at(msg, pos)?;
            Ok(RData::Name(host))
        }
        TYPE_SIG => {
            ensure(pos, 18, rrend)?;
            let (signer, signer_end) = name_at(msg, pos + 18)?;
            if signer_end > rrend {
                return Err(Error::BadPacket);
            }
            Ok(RData::Sig {
                type_covered: be16(msg, pos),
                algorithm: msg[pos + 2],
                labels: msg[pos + 3],
                original_ttl: be32(msg, pos + 4),
                expiration: be32(msg, pos + 8),
                time_signed: be32(msg, pos + 12),
                key_tag: be16(msg, pos + 16),
                signer,
                signature: msg[signer_end..rrend].to_vec(),
            })
        }
        TYPE_SOA => {
            let (primary, next) = name_at(msg, pos)?;
            let (admin, next) = name_at(msg, next)?;
            ensure(next, 20, rrend)?;
            Ok(RData::Soa {
                primary,
                admin,
                serial: be32(msg, next),
                refresh: be32(msg, next + 4),
                retry: be32(msg, next + 8),
                expire: be32(msg, next + 12),
                minimum: be32(msg, next + 16),
            })
        }
        TYPE_SRV => {
            ensure(pos, 6, rrend)?;
            let (target, _) = name_at(msg, pos + 6)?;
            Ok(RData::Srv {
                priority: be16(msg, pos),
                weight: be16(msg, pos + 2),
                port: be16(msg, pos + 4),
                target,
            })
        }
        TYPE_HINFO | TYPE_ISDN | TYPE_X25 | TYPE_TEXT => {
            let mut strings = Vec::new();
            let mut at = pos;
            while at < rrend {
                let n = usize::from(msg[at]);
                ensure(at + 1, n, rrend)?;
                strings.push(until_nul(&msg[at + 1..at + 1 + n]).to_vec());
                at += n + 1;
            }
            Ok(RData::Strings(strings))
        }
        _ => Err(Error::NotImplemented),
    }
}
