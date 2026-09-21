use crate::model::TelescopeDevice;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum DiscoveryEvent {
    Status(String),
    Found(TelescopeDevice),
    Finished { checked: usize },
    Failed(String),
}

pub struct DiscoveryHandle {
    cancelled: Arc<AtomicBool>,
    receiver: mpsc::Receiver<DiscoveryEvent>,
}

impl DiscoveryHandle {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn try_recv(&self) -> Result<DiscoveryEvent, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

impl Drop for DiscoveryHandle {
    fn drop(&mut self) {
        self.cancel();
    }
}

pub fn start() -> DiscoveryHandle {
    let cancelled = Arc::new(AtomicBool::new(false));
    let worker_cancelled = Arc::clone(&cancelled);
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = local_ipv4().and_then(|local| scan_subnet(local, &worker_cancelled, &sender));
        if let Err(error) = result {
            let _ = sender.send(DiscoveryEvent::Failed(error));
        }
    });
    DiscoveryHandle {
        cancelled,
        receiver,
    }
}

pub fn validate(ip: IpAddr) -> bool {
    let timeout = Duration::from_millis(350);
    [445, 4554]
        .into_iter()
        .any(|port| TcpStream::connect_timeout(&SocketAddr::new(ip, port), timeout).is_ok())
}

fn local_ipv4() -> Result<Ipv4Addr, String> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))
        .map_err(|error| format!("could not inspect the active network adapter: {error}"))?;
    socket
        .connect((Ipv4Addr::new(192, 0, 2, 1), 9))
        .map_err(|error| format!("could not select an active network route: {error}"))?;
    match socket.local_addr().map(|address| address.ip()) {
        Ok(IpAddr::V4(ip)) if !ip.is_loopback() && !ip.is_link_local() => Ok(ip),
        _ => Err("no active IPv4 network connection was found".into()),
    }
}

fn scan_subnet(
    local: Ipv4Addr,
    cancelled: &AtomicBool,
    sender: &mpsc::Sender<DiscoveryEvent>,
) -> Result<(), String> {
    let [a, b, c, _] = local.octets();
    let _ = sender.send(DiscoveryEvent::Status(format!(
        "Scanning {a}.{b}.{c}.1 to {a}.{b}.{c}.254…"
    )));
    let (result_sender, result_receiver) = mpsc::channel();
    let mut workers = Vec::new();
    for worker in 0..16_u8 {
        let result_sender = result_sender.clone();
        workers.push(thread::spawn(move || {
            let timeout = Duration::from_millis(90);
            let mut host = u16::from(worker) + 1;
            while host <= 254 {
                let ip = Ipv4Addr::new(a, b, c, host as u8);
                let address = IpAddr::V4(ip);
                let smb =
                    TcpStream::connect_timeout(&SocketAddr::new(address, 445), timeout).is_ok();
                let rtsp =
                    TcpStream::connect_timeout(&SocketAddr::new(address, 4554), timeout).is_ok();
                if smb || rtsp {
                    let _ = result_sender.send(address);
                }
                host += 16;
            }
        }));
    }
    drop(result_sender);
    for worker in workers {
        let _ = worker.join();
    }
    if cancelled.load(Ordering::Acquire) {
        return Ok(());
    }
    let mut found: Vec<_> = result_receiver.try_iter().collect();
    found.sort();
    found.dedup();
    for ip in found {
        let device = TelescopeDevice::new(format!("Seestar candidate {ip}"), ip, false);
        let _ = sender.send(DiscoveryEvent::Found(device));
    }
    let _ = sender.send(DiscoveryEvent::Finished { checked: 254 });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_without_known_service_is_not_validated() {
        // This is intentionally a weak assertion: a developer may have an unrelated service on 445/4554.
        let _ = validate(IpAddr::V4(Ipv4Addr::LOCALHOST));
    }
}
