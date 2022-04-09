/// Taken and modified `form_urlencoded::Parse` and does not decode output
#[derive(Copy, Clone)]
pub struct Parse<'a> {
    pub input: &'a [u8],
}

impl<'a> Iterator for Parse<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input.is_empty() {
                return None;
            }
            let mut split2 = self.input.splitn(2, |&b| b == b'&');
            let sequence = split2.next().unwrap();
            self.input = split2.next().unwrap_or(&[]);
            if sequence.is_empty() {
                continue;
            }
            let mut split2 = sequence.splitn(2, |&b| b == b'=');
            let name = split2.next().unwrap();
            let value = split2.next().unwrap_or(&[]);
            return Some((name, value));
        }
    }
}

pub fn query_pairs(input: &[u8]) -> Parse {
    Parse { input }
}

/// Taken and modified from `percent_encoding::PercentDecode`. Allows access to the underlying
/// slice to get the remaining bytes
pub struct PercentDecode<'a> {
    pub bytes: std::slice::Iter<'a, u8>,
}

impl<'a> PercentDecode<'a> {
    pub fn peek(&self) -> Option<&u8> {
        self.bytes.as_slice().get(0)
    }

    pub fn consume_while<P>(&mut self, predicate: P)
    where
        P: Fn(u8) -> bool,
    {
        while let Some(next) = self.peek() {
            if predicate(*next) {
                self.next();
            } else {
                break;
            }
        }
    }
}

fn after_percent_sign(iter: &mut std::slice::Iter<u8>) -> Option<u8> {
    let mut cloned_iter = iter.clone();
    let h = char::from(*cloned_iter.next()?).to_digit(16)?;
    let l = char::from(*cloned_iter.next()?).to_digit(16)?;
    *iter = cloned_iter;
    Some(h as u8 * 0x10 + l as u8)
}

impl<'a> Iterator for PercentDecode<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.bytes.next().map(|&byte| {
            if byte == b'%' {
                after_percent_sign(&mut self.bytes).unwrap_or(byte)
            } else {
                byte
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let bytes = self.bytes.len();
        (bytes / 3, Some(bytes))
    }
}

pub fn percent_decode(slice: &[u8]) -> PercentDecode {
    PercentDecode {
        bytes: slice.iter(),
    }
}
