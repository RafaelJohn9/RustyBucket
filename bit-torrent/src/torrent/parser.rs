use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Bencode {
    Int(i64),
    Bytes(Vec<u8>),
    List(Vec<BValue>),
    Dict(BTreeMap<Vec<u8>, BValue>),
}

#[derive(Debug)]
pub struct BValue {
    pub value: Bencode,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct FileEntry {
    pub length: i64,
    pub path: Vec<String>,
}

#[derive(Debug)]
pub struct Info {
    pub name: Option<String>,
    pub piece_length: Option<i64>,
    pub pieces: Option<Vec<u8>>,
    pub length: Option<i64>,
    pub files: Vec<FileEntry>,
    pub raw: Vec<u8>, // raw bencoded info dict (for computing info_hash)
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: Option<String>,
    pub announce_list: Vec<String>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub creation_date: Option<i64>,
    pub info: Info,
}

type ParseResult<T> = Result<T, String>;

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        Parser { input, pos: 0 }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<u8> {
        if self.eof() {
            None
        } else {
            let b = self.input[self.pos];
            self.pos += 1;
            Some(b)
        }
    }

    fn parse_bvalue(&mut self) -> ParseResult<BValue> {
        let start = self.pos;
        let val = match self.peek() {
            Some(b'i') => {
                self.next();
                let int = self.parse_int()?;
                Bencode::Int(int)
            }
            Some(b'l') => {
                self.next();
                let mut items = Vec::new();
                while let Some(b) = self.peek() {
                    if b == b'e' {
                        self.next();
                        break;
                    }
                    items.push(self.parse_bvalue()?);
                }
                Bencode::List(items)
            }
            Some(b'd') => {
                self.next();
                let mut map = BTreeMap::new();
                while let Some(b) = self.peek() {
                    if b == b'e' {
                        self.next();
                        break;
                    }
                    let key = self.parse_bytes_raw()?;
                    let key_clone = key.clone();
                    let val = self.parse_bvalue()?;
                    map.insert(key_clone, val);
                }
                Bencode::Dict(map)
            }
            Some(d) if (b'0'..=b'9').contains(&d) => {
                let bytes = self.parse_bytes()?;
                Bencode::Bytes(bytes)
            }
            Some(_) => return Err(format!("Invalid token at position {}", self.pos)),
            None => return Err("Unexpected EOF while parsing bencode".to_string()),
        };
        let end = self.pos;
        Ok(BValue { value: val, start, end })
    }

    fn parse_int(&mut self) -> ParseResult<i64> {
        // read until 'e'
        let mut negative = false;
        if self.peek() == Some(b'-') {
            negative = true;
            self.next();
        }
        let mut num: i64 = 0;
        let mut digits = 0;
        while let Some(&c) = self.input.get(self.pos) {
            if c == b'e' {
                self.next();
                break;
            }
            if !(b'0'..=b'9').contains(&c) {
                return Err(format!("Invalid integer char '{}' at {}", c as char, self.pos));
            }
            num = num
                .checked_mul(10)
                .and_then(|n| n.checked_add((c - b'0') as i64))
                .ok_or_else(|| "Integer overflow".to_string())?;
            digits += 1;
            self.pos += 1;
        }
        if digits == 0 {
            return Err("Invalid integer with no digits".to_string());
        }
        if negative {
            num = -num;
        }
        Ok(num)
    }

    fn parse_bytes(&mut self) -> ParseResult<Vec<u8>> {
        let len = self.parse_usize_prefix()?;
        if self.pos + len > self.input.len() {
            return Err("Truncated byte string".to_string());
        }
        let slice = &self.input[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice.to_vec())
    }

    fn parse_bytes_raw(&mut self) -> ParseResult<Vec<u8>> {
        // same as parse_bytes but returns raw bytes (key) without UTF-8 conversion
        self.parse_bytes()
    }

    fn parse_usize_prefix(&mut self) -> ParseResult<usize> {
        // read decimal digits until ':'
        let mut num: usize = 0;
        let mut digits = 0;
        while let Some(&c) = self.input.get(self.pos) {
            if c == b':' {
                self.pos += 1;
                break;
            }
            if !(b'0'..=b'9').contains(&c) {
                return Err(format!("Invalid string length char '{}' at {}", c as char, self.pos));
            }
            num = num
                .checked_mul(10)
                .and_then(|n| n.checked_add((c - b'0') as usize))
                .ok_or_else(|| "String length overflow".to_string())?;
            digits += 1;
            self.pos += 1;
        }
        if digits == 0 {
            return Err("Missing string length".to_string());
        }
        Ok(num)
    }
}

pub fn parse_torrent(input: &[u8]) -> ParseResult<Torrent> {
    let mut p = Parser::new(input);
    let top = p.parse_bvalue()?;
    let info_bvalue = match top.value {
        Bencode::Dict(mut map) => {
            // extract announce, announce-list, comment, created by, creation date
            let announce = map
                .get(b"announce".as_ref())
                .and_then(|bv| as_string(&bv.value).ok());
            let announce_list = map
                .get(b"announce-list".as_ref())
                .and_then(|bv| as_list_of_strings(&bv.value).ok())
                .unwrap_or_default();
            let comment = map
                .get(b"comment".as_ref())
                .and_then(|bv| as_string(&bv.value).ok());
            let created_by = map
                .get(b"created by".as_ref())
                .and_then(|bv| as_string(&bv.value).ok());
            let creation_date = map
                .get(b"creation date".as_ref())
                .and_then(|bv| as_int(&bv.value).ok());

            let info_bv = map
                .remove(b"info".as_ref())
                .ok_or_else(|| "Missing 'info' dictionary".to_string())?;

            let info = extract_info(&info_bv, input)?;
            Ok(Torrent {
                announce,
                announce_list,
                comment,
                created_by,
                creation_date,
                info,
            })
        }
        _ => Err("Top-level bencode is not a dictionary".to_string()),
    }?;
    Ok(info_bvalue)
}

fn as_string(b: &Bencode) -> ParseResult<String> {
    if let Bencode::Bytes(v) = b {
        String::from_utf8(v.clone()).map_err(|e| format!("Invalid UTF-8 string: {}", e))
    } else {
        Err("Expected byte string".to_string())
    }
}

fn as_int(b: &Bencode) -> ParseResult<i64> {
    if let Bencode::Int(i) = b {
        Ok(*i)
    } else {
        Err("Expected integer".to_string())
    }
}

fn as_list_of_strings(b: &Bencode) -> ParseResult<Vec<String>> {
    if let Bencode::List(items) = b {
        let mut out = Vec::new();
        for item in items {
            match &item.value {
                Bencode::Bytes(v) => {
                    out.push(String::from_utf8(v.clone()).map_err(|e| format!("Invalid UTF-8: {}", e))?);
                }
                Bencode::List(sub) => {
                    // allow announce-list style nested lists
                    for s in sub {
                        if let Bencode::Bytes(v2) = &s.value {
                            out.push(String::from_utf8(v2.clone()).map_err(|e| format!("Invalid UTF-8: {}", e))?);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(out)
    } else {
        Err("Expected list".to_string())
    }
}

fn extract_info(info_bv: &BValue, input: &[u8]) -> ParseResult<Info> {
    // info_bv contains the parsed info dict and its start/end in input so we can get raw bytes
    let raw = input
        .get(info_bv.start..info_bv.end)
        .ok_or_else(|| "Failed to slice raw info bytes".to_string())?
        .to_vec();

    let info_map = match &info_bv.value {
        Bencode::Dict(m) => m,
        _ => return Err("info is not a dict".to_string()),
    };

    let name = info_map
        .get(b"name".as_ref())
        .and_then(|bv| match &bv.value {
            Bencode::Bytes(v) => String::from_utf8(v.clone()).ok(),
            _ => None,
        });

    let piece_length = info_map
        .get(b"piece length".as_ref())
        .and_then(|bv| match &bv.value {
            Bencode::Int(i) => Some(*i),
            _ => None,
        });

    let pieces = info_map
        .get(b"pieces".as_ref())
        .and_then(|bv| match &bv.value {
            Bencode::Bytes(v) => Some(v.clone()),
            _ => None,
        });

    let length = info_map
        .get(b"length".as_ref())
        .and_then(|bv| match &bv.value {
            Bencode::Int(i) => Some(*i),
            _ => None,
        });

    let mut files = Vec::new();
    if let Some(bv) = info_map.get(b"files".as_ref()) {
        if let Bencode::List(list) = &bv.value {
            for entry in list {
                if let Bencode::Dict(d) = &entry.value {
                    let flen = d
                        .get(b"length".as_ref())
                        .and_then(|bv2| match &bv2.value {
                            Bencode::Int(i) => Some(*i),
                            _ => None,
                        })
                        .ok_or_else(|| "file entry missing length".to_string())?;
                    let path_vec = d
                        .get(b"path".as_ref())
                        .and_then(|bv2| match &bv2.value {
                            Bencode::List(pv) => {
                                let mut parts = Vec::new();
                                for p in pv {
                                    if let Bencode::Bytes(bs) = &p.value {
                                        if let Ok(s) = String::from_utf8(bs.clone()) {
                                            parts.push(s);
                                        }
                                    }
                                }
                                Some(parts)
                            }
                            _ => None,
                        })
                        .ok_or_else(|| "file entry missing path".to_string())?;
                    files.push(FileEntry {
                        length: flen,
                        path: path_vec,
                    });
                }
            }
        }
    }

    Ok(Info {
        name,
        piece_length,
        pieces,
        length,
        files,
        raw,
    })
}