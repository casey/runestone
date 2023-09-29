use super::*;

#[cfg(test)]
pub fn encode(mut n: u128) -> Vec<u8> {
  let mut out = Vec::new();

  loop {
    let mut byte = n as u8 % 128;

    if !out.is_empty() {
      byte |= 0b1000_0000;
    }

    out.push(byte);

    if n < 128 {
      break;
    }

    n = n / 128 - 1;
  }

  out.reverse();

  out
}

pub fn decode(buffer: &[u8]) -> Result<(u128, usize)> {
  let mut n = 0;
  let mut i = 0;

  loop {
    let b = buffer.get(i).cloned().unwrap() as u128;

    if b < 128 {
      return Ok((n + b, i + 1));
    }

    n += b - 127;

    n = n.checked_mul(128).ok_or(Error::Varint)?;

    i += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn powers_of_two_round_trip_successfully() {
    for i in 0..128 {
      let n = 1 << i;
      let encoded = encode(n);
      let (decoded, length) = decode(&encoded).unwrap();
      assert_eq!(decoded, n);
      assert_eq!(length, encoded.len());
    }
  }

  #[test]
  fn alternating_bit_strings_round_trip_successfully() {
    let mut n = 0;

    for i in 0..129 {
      n = n << 1 | (i % 2);
      let encoded = encode(n);
      let (decoded, length) = decode(&encoded).unwrap();
      assert_eq!(decoded, n);
      assert_eq!(length, encoded.len());
    }
  }

  #[test]
  fn decoding_integer_over_max_is_an_error() {
    assert_eq!(
      decode(&[
        130, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 255,
        0,
      ]),
      Err(Error::Varint)
    );
  }

  #[test]
  fn decoding_bip_samples() {
    assert_eq!(decode(&[0x00]), Ok((0, 1)));
    assert_eq!(decode(&[0x01]), Ok((1, 1)));
    assert_eq!(decode(&[0x7F]), Ok((127, 1)));
    assert_eq!(decode(&[0x80, 0x00]), Ok((128, 2)));
    assert_eq!(decode(&[0x80, 0x7F]), Ok((255, 2)));
    assert_eq!(decode(&[0x81, 0x00]), Ok((256, 2)));
    assert_eq!(decode(&[0xFE, 0x7F]), Ok((16383, 2)));
    assert_eq!(decode(&[0xFF, 0x00]), Ok((16384, 2)));
    assert_eq!(decode(&[0xFF, 0x7F]), Ok((16511, 2)));
    assert_eq!(decode(&[0x82, 0xFE, 0x7F]), Ok((65535, 3)));
    assert_eq!(decode(&[0x8E, 0xFE, 0xFE, 0xFF, 0x00, ]), Ok((1 << 32, 5)));
  }

  #[test]
  fn encoding_bip_samples() {
    assert_eq!(encode(0), &[0x00]);
    assert_eq!(encode(1), &[0x01]);
    assert_eq!(encode(127), &[0x7F]);
    assert_eq!(encode(128), &[0x80, 0x00]);
    assert_eq!(encode(255), &[0x80, 0x7F]);
    assert_eq!(encode(256), &[0x81, 0x00]);
    assert_eq!(encode(16383), &[0xFE, 0x7F]);
    assert_eq!(encode(16384), &[0xFF, 0x00]);
    assert_eq!(encode(16511), &[0xFF, 0x7F]);
    assert_eq!(encode(65535), &[0x82, 0xFE, 0x7F]);
    assert_eq!(encode(1 << 32), &[0x8E, 0xFE, 0xFE, 0xFF, 0x00, ]);
  }
}
