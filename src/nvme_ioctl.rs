use linux_nvme_sys::{nvme_admin_cmd, nvme_admin_opcode, nvme_id_ctrl, nvme_ioctl_admin_cmd};
use std::fs;
use std::mem::MaybeUninit;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::Path;
use std::ptr::NonNull;

fn nvme_ioctl<P: AsRef<Path>, F: FnMut(RawFd) -> nix::Result<std::os::raw::c_int>>(
    dev: P,
    mut cmd: F,
) -> std::io::Result<()> {
    let file = fs::File::open(dev)?;
    let raw_fd = file.as_raw_fd();
    let _ = cmd(raw_fd).map_err(|err| std::io::Error::from(err.as_errno().unwrap()))?;
    Ok(())
}

unsafe fn nvme_admin_cmd_submit<P: AsRef<Path>, T: Sized>(
    dev: P,
    id_response: NonNull<T>,
) -> std::io::Result<()> {
    let mut nvme_admin_command = nvme_admin_cmd {
        opcode: nvme_admin_opcode::nvme_admin_identify as _,
        addr: id_response.as_ptr() as _,
        data_len: std::mem::size_of::<T>() as _,
        cdw10: 1,
        ..nvme_admin_cmd::default()
    };
    nvme_ioctl(dev, |fd| nvme_ioctl_admin_cmd(fd, &mut nvme_admin_command))
}

pub fn nvme_identify_controller<P: AsRef<Path>>(dev: P) -> std::io::Result<nvme_id_ctrl> {
    unsafe {
        let mut id_ctrl = MaybeUninit::<nvme_id_ctrl>::zeroed();
        nvme_admin_cmd_submit(dev, NonNull::new_unchecked(id_ctrl.as_mut_ptr()))?;
        Ok(id_ctrl.assume_init())
    }
}

pub fn i8_as_u8<'a>(buf: &'a [i8]) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) }
}
