use linux_nvme_sys::{nvme_admin_opcode, nvme_id_ctrl, nvme_passthru_cmd};
use std::fs;
use std::mem::MaybeUninit;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::ptr::NonNull;

const NVME_IOCTL_ADMIN_CMD: u64 = 0xc0484e41;

pub unsafe fn nvme_ioctl<P: AsRef<Path>, T: Sized>(
    dev: P,
    id_response: NonNull<T>,
) -> std::io::Result<()> {
    let file = fs::File::open(dev)?;
    let raw_fd = file.as_raw_fd();

    let mut nvme_admin_command = nvme_passthru_cmd {
        opcode: nvme_admin_opcode::nvme_admin_identify as u8,
        addr: id_response.as_ptr() as u64,
        data_len: std::mem::size_of::<T>() as u32,
        cdw10: 1,
        ..nvme_passthru_cmd::default()
    };
    let res = linux_nvme_sys::ioctl(
        raw_fd,
        NVME_IOCTL_ADMIN_CMD,
        &mut nvme_admin_command as *mut nvme_passthru_cmd as u64,
    );
    if res != 0 {
        Err(std::io::Error::last_os_error())?;
    }
    Ok(())
}

pub fn nvme_identify_controller<P: AsRef<Path>>(dev: P) -> std::io::Result<nvme_id_ctrl> {
    unsafe {
        let mut id_ctrl = MaybeUninit::<nvme_id_ctrl>::zeroed();
        nvme_ioctl(dev, NonNull::new_unchecked(id_ctrl.as_mut_ptr()))?;
        Ok(id_ctrl.assume_init())
    }
}

pub fn i8_to_u8<'a>(buf: &'a [i8]) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) }
}
