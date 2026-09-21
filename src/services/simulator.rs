use crate::model::TelescopeDevice;
use std::fs;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{Path, PathBuf};

pub fn provision(root: &Path) -> io::Result<Vec<TelescopeDevice>> {
    if root.exists() {
        fs::remove_dir_all(root)?;
    }
    for device in ["node-01", "node-02"] {
        let source = root.join(device).join("EMMC Images").join("MyWorks");
        let m42 = source.join("M42");
        let jupiter = source.join("Jupiter");
        fs::create_dir_all(&m42)?;
        fs::create_dir_all(&jupiter)?;
        fs::write(m42.join("M42_0001.fit"), synthetic_fits(32, 24, 1))?;
        fs::write(m42.join("M42_0002.fit"), synthetic_fits(32, 24, 2))?;
        fs::write(m42.join("M42_preview.jpg"), b"simulated-jpeg-fixture")?;
        fs::write(
            jupiter.join("Jupiter_capture.mp4"),
            b"simulated-video-fixture",
        )?;
    }

    Ok(vec![
        TelescopeDevice::new(
            "Seestar-Simulator-01",
            IpAddr::V4(Ipv4Addr::new(192, 0, 2, 41)),
            true,
        )
        .with_source_path(root.join("node-01").join("EMMC Images").join("MyWorks")),
        TelescopeDevice::new(
            "Seestar-Simulator-02",
            IpAddr::V4(Ipv4Addr::new(192, 0, 2, 42)),
            true,
        )
        .with_source_path(root.join("node-02").join("EMMC Images").join("MyWorks")),
    ])
}

fn synthetic_fits(width: usize, height: usize, phase: u16) -> Vec<u8> {
    let mut bytes = Vec::new();
    for text in [
        "SIMPLE  =                    T",
        "BITPIX  =                   16",
        "NAXIS   =                    2",
        &format!("NAXIS1  = {width:20}"),
        &format!("NAXIS2  = {height:20}"),
        "BZERO   =                32768",
        "BSCALE  =                    1",
        "OBJECT  = 'SIMULATED SEESTAR'",
        "END",
    ] {
        let mut card = [b' '; 80];
        let text = text.as_bytes();
        card[..text.len().min(80)].copy_from_slice(&text[..text.len().min(80)]);
        bytes.extend_from_slice(&card);
    }
    bytes.resize(bytes.len().div_ceil(2880) * 2880, b' ');
    for y in 0..height {
        for x in 0..width {
            let gradient = ((x * 1300 + y * 700 + usize::from(phase) * 900) % 65535) as u16;
            let signed = gradient.wrapping_sub(32768) as i16;
            bytes.extend_from_slice(&signed.to_be_bytes());
        }
    }
    bytes
}

pub fn default_root() -> PathBuf {
    std::env::temp_dir().join("rseestar-simulator")
}
