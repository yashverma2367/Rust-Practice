use std::io::{self, Read};

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.input.read(buf)?;
        println!("n: {}", n);
        for byte in &mut buf[..n] {
            *byte = match *byte {
                b'a'..=b'z' => {
                    let base = b'a';
                    ((*byte - base + self.rot) % 26) + base
                }
                b'A'..=b'Z' => {
                    let base = b'A';
                    ((*byte - base + self.rot) % 26) + base
                }
                _ => *byte,
            };
        }

        Ok(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_slice(),
            rot: 13,
        };
        let mut buf = [0u8; 256];
        let already_read = rot.read(&mut buf);
        println!("==============read: {:?}", already_read);
        assert_eq!(already_read.unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}

fn main() {
    println!("Run tests with: cargo test");
}