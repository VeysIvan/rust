use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
      match self.input.read(buf) {
          Ok(read_bytes) => {
              for i in 0..read_bytes {
                  let c = buf[i];
                  if c.is_ascii_alphabetic() {
                      let ascii_offset = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                      buf[i] = ((c - ascii_offset + self.rot) % 26 + ascii_offset) as u8;
                  }
              }
              Ok(read_bytes)
          }
          Err(e) => Err(e),
      }
  }
}

fn main() {
  let mut rot =
      RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
  let mut result = String::new();
  rot.read_to_string(&mut result).unwrap();
  println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}