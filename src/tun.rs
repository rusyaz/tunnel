use std::{fs::{self, OpenOptions}, os::fd::AsRawFd};

use nix::ioctl_write_ptr_bad;

ioctl_write_ptr_bad!(tunsetiff, libc::TUNSETIFF, libc::ifreq);

pub struct Tun {
  name: String,
  file: fs::File,
}

impl Tun {
  pub fn new(name: String) -> anyhow::Result<Self> {
    if name.len() >= libc::IFNAMSIZ {
      anyhow::bail!("Name of tunnel is longer than must be.");
    }

    let file = OpenOptions::new()
      .read(true)
      .write(true)
      .open("/dev/net/tun")?;

    let mut ifr: libc::ifreq = unsafe { std::mem::zeroed() };
    ifr.ifr_ifru.ifru_flags = (libc::IFF_TUN | libc::IFF_NO_PI) as libc::c_short;
    for (d, s) in ifr.ifr_name.iter_mut().zip(name.bytes()) {
      *d = s as libc::c_char;
    }
    unsafe { tunsetiff(file.as_raw_fd(),&ifr)?; }
    Ok(Self { name, file })
  }
}
