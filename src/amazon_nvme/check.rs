use linux_nvme_sys::nvme_id_ctrl;

const AMZN_NVME_VID: u16 = 0x1D0F;
const AMZN_NVME_EBS_MN: &'static [u8; 26] = b"Amazon Elastic Block Store";

pub fn is_amazon_device(id_ctrl: &nvme_id_ctrl) -> bool {
    id_ctrl.vid == AMZN_NVME_VID && is_matching_amzn_nvme_ebs_mn(&id_ctrl.mn)
}

fn is_matching_amzn_nvme_ebs_mn<T>(buf: &[T; 40]) -> bool
where
    T: AsU8,
{
    buf.iter()
        .map(|&e| AsU8::as_u8(e))
        .zip(AMZN_NVME_EBS_MN.iter().chain(std::iter::repeat(&b' ')))
        .all(|(a, &b)| a == b)
}

trait AsU8: Copy {
    fn as_u8(self) -> u8;
}

impl AsU8 for u8 {
    fn as_u8(self) -> u8 {
        self
    }
}

impl AsU8 for i8 {
    fn as_u8(self) -> u8 {
        self as u8
    }
}

#[test]
fn test_is_matching_amzn_nvme_ebs_mn() {
    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"Amazon Elastic Block Store              "),
        true
    );

    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"                                        "),
        false
    );
    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"Amazon Elastic Block Stor               "),
        false
    );
    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"Amazon Elastic Block StoreA             "),
        false
    );
    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"test                                    "),
        false
    );
    assert_eq!(
        is_matching_amzn_nvme_ebs_mn(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        false
    );
}
