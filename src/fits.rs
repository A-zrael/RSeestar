use std::collections::BTreeMap;
use std::fmt;

const BLOCK_SIZE: usize = 2880;
const CARD_SIZE: usize = 80;

#[derive(Clone, Debug)]
pub struct FitsImage {
    pub width: usize,
    pub height: usize,
    pub bitpix: i32,
    pub bzero: f64,
    pub bscale: f64,
    pub headers: BTreeMap<String, String>,
    pub pixels: Vec<f32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FitsError {
    MissingEnd,
    MissingKeyword(&'static str),
    InvalidKeyword(&'static str),
    UnsupportedBitpix(i32),
    Truncated,
    Overflow,
}

impl fmt::Display for FitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnd => f.write_str("FITS header has no END card"),
            Self::MissingKeyword(key) => write!(f, "FITS header is missing {key}"),
            Self::InvalidKeyword(key) => write!(f, "FITS header contains an invalid {key}"),
            Self::UnsupportedBitpix(value) => write!(f, "unsupported FITS BITPIX value {value}"),
            Self::Truncated => f.write_str("FITS pixel data is truncated"),
            Self::Overflow => f.write_str("FITS dimensions overflow the address space"),
        }
    }
}

impl std::error::Error for FitsError {}

impl FitsImage {
    pub fn parse(bytes: &[u8]) -> Result<Self, FitsError> {
        let (headers, data_offset) = parse_header(bytes)?;
        let width = parse_required::<usize>(&headers, "NAXIS1")?;
        let height = parse_required::<usize>(&headers, "NAXIS2")?;
        let bitpix = parse_required::<i32>(&headers, "BITPIX")?;
        let bzero = parse_optional::<f64>(&headers, "BZERO").unwrap_or(0.0);
        let bscale = parse_optional::<f64>(&headers, "BSCALE").unwrap_or(1.0);
        let count = width.checked_mul(height).ok_or(FitsError::Overflow)?;
        let bytes_per_pixel = match bitpix {
            8 => 1,
            16 => 2,
            32 | -32 => 4,
            64 | -64 => 8,
            other => return Err(FitsError::UnsupportedBitpix(other)),
        };
        let data_len = count
            .checked_mul(bytes_per_pixel)
            .ok_or(FitsError::Overflow)?;
        let data = bytes
            .get(
                data_offset
                    ..data_offset
                        .checked_add(data_len)
                        .ok_or(FitsError::Overflow)?,
            )
            .ok_or(FitsError::Truncated)?;
        let mut pixels = Vec::with_capacity(count);
        for index in 0..count {
            let offset = index * bytes_per_pixel;
            let raw = match bitpix {
                8 => f64::from(data[offset]),
                16 => f64::from(i16::from_be_bytes([data[offset], data[offset + 1]])),
                32 => f64::from(i32::from_be_bytes(
                    data[offset..offset + 4].try_into().unwrap(),
                )),
                64 => i64::from_be_bytes(data[offset..offset + 8].try_into().unwrap()) as f64,
                -32 => f64::from(f32::from_be_bytes(
                    data[offset..offset + 4].try_into().unwrap(),
                )),
                -64 => f64::from_be_bytes(data[offset..offset + 8].try_into().unwrap()),
                _ => unreachable!(),
            };
            pixels.push((raw * bscale + bzero) as f32);
        }
        Ok(Self {
            width,
            height,
            bitpix,
            bzero,
            bscale,
            headers,
            pixels,
        })
    }

    pub fn preview_rgba(&self, stretch: f32) -> Vec<u8> {
        let mut sample: Vec<f32> = self
            .pixels
            .iter()
            .copied()
            .filter(|v| v.is_finite())
            .collect();
        sample.sort_by(|a, b| a.total_cmp(b));
        if sample.is_empty() {
            return vec![0; self.width * self.height * 4];
        }
        let low_index = ((sample.len() - 1) as f32 * 0.005) as usize;
        let high_quantile = (0.995 - (stretch.clamp(0.0, 10.0) as f64 * 0.035)).clamp(0.55, 0.995);
        let high_index = ((sample.len() - 1) as f64 * high_quantile) as usize;
        let low = sample[low_index];
        let high = sample[high_index].max(low + f32::EPSILON);
        let gamma = (1.0 - stretch.clamp(0.0, 10.0) * 0.075).max(0.2);
        let mut rgba = vec![0; self.width * self.height * 4];
        for y in 0..self.height {
            let source_y = self.height - 1 - y;
            for x in 0..self.width {
                let source = source_y * self.width + x;
                let target = (y * self.width + x) * 4;
                let normalized = ((self.pixels[source] - low) / (high - low)).clamp(0.0, 1.0);
                let value = (normalized.powf(gamma) * 255.0).round() as u8;
                rgba[target..target + 4].copy_from_slice(&[value, value, value, 255]);
            }
        }
        rgba
    }
}

fn parse_header(bytes: &[u8]) -> Result<(BTreeMap<String, String>, usize), FitsError> {
    let mut headers = BTreeMap::new();
    for (index, card) in bytes.as_chunks::<CARD_SIZE>().0.iter().enumerate() {
        let keyword = String::from_utf8_lossy(&card[..8]).trim().to_string();
        if keyword == "END" {
            let consumed = (index + 1) * CARD_SIZE;
            let offset = consumed.div_ceil(BLOCK_SIZE) * BLOCK_SIZE;
            if offset > bytes.len() {
                return Err(FitsError::Truncated);
            }
            return Ok((headers, offset));
        }
        if keyword.is_empty() || card.get(8) != Some(&b'=') {
            continue;
        }
        let value_area = String::from_utf8_lossy(&card[10..]);
        let value = value_area
            .split('/')
            .next()
            .unwrap_or_default()
            .trim()
            .trim_matches('\'')
            .trim()
            .to_string();
        headers.insert(keyword, value);
    }
    Err(FitsError::MissingEnd)
}

fn parse_required<T: std::str::FromStr>(
    headers: &BTreeMap<String, String>,
    key: &'static str,
) -> Result<T, FitsError> {
    headers
        .get(key)
        .ok_or(FitsError::MissingKeyword(key))?
        .parse()
        .map_err(|_| FitsError::InvalidKeyword(key))
}

fn parse_optional<T: std::str::FromStr>(
    headers: &BTreeMap<String, String>,
    key: &str,
) -> Option<T> {
    headers.get(key)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(text: &str) -> [u8; CARD_SIZE] {
        let mut value = [b' '; CARD_SIZE];
        value[..text.len()].copy_from_slice(text.as_bytes());
        value
    }

    fn sample_fits() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&card("SIMPLE  =                    T"));
        bytes.extend_from_slice(&card("BITPIX  =                   16"));
        bytes.extend_from_slice(&card("NAXIS   =                    2"));
        bytes.extend_from_slice(&card("NAXIS1  =                    2"));
        bytes.extend_from_slice(&card("NAXIS2  =                    2"));
        bytes.extend_from_slice(&card("BZERO   =                32768"));
        bytes.extend_from_slice(&card("END"));
        bytes.resize(BLOCK_SIZE, b' ');
        for value in [-32768_i16, -1, 0, 32767] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        bytes
    }

    #[test]
    fn parses_scaled_16_bit_image() {
        let image = FitsImage::parse(&sample_fits()).unwrap();
        assert_eq!((image.width, image.height, image.bitpix), (2, 2, 16));
        assert_eq!(image.pixels, [0.0, 32767.0, 32768.0, 65535.0]);
    }

    #[test]
    fn preview_is_rgba_and_vertically_flipped() {
        let image = FitsImage::parse(&sample_fits()).unwrap();
        let preview = image.preview_rgba(0.0);
        assert_eq!(preview.len(), 16);
        assert!(preview[0] > preview[8]);
        assert_eq!(preview[3], 255);
    }

    #[test]
    fn rejects_a_truncated_image() {
        let mut bytes = sample_fits();
        bytes.truncate(BLOCK_SIZE + 2);
        assert_eq!(FitsImage::parse(&bytes).unwrap_err(), FitsError::Truncated);
    }
}
