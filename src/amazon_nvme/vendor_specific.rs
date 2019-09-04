use linux_nvme_sys::{__u8, nvme_id_ctrl};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AmznVS {
    pub bdev: [::std::os::raw::c_char; 32usize], // block device name
}

impl AmznVS {
    pub fn from_nvme_id_ctrl_vs<'a>(buf: &'a [__u8; 1024usize]) -> &'a Self {
        unsafe { std::mem::transmute(buf) }
    }

    pub fn from_nvme_id_ctrl<'a>(id_ctrl: &'a nvme_id_ctrl) -> &'a Self {
        Self::from_nvme_id_ctrl_vs(&id_ctrl.vs)
    }
}
