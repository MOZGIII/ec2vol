mod amazon_nvme;
mod nvme_ioctl;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args_os();
    let exename = args.next().unwrap().into_string().unwrap();
    let dev = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: {} /dev/nvme0n1", exename);
            std::process::exit(1);
        }
    };
    let val = nvme_ioctl::nvme_identify_controller(dev)?;

    if !amazon_nvme::is_amazon_device(&val) {
        eprintln!(
            "Not an Amazon device: {}",
            String::from_utf8_lossy(nvme_ioctl::i8_as_u8(&val.mn)).trim_end()
        );
        std::process::exit(2)
    }

    println!("{}", String::from_utf8_lossy(nvme_ioctl::i8_as_u8(&val.sn)));
    println!(
        "{}",
        String::from_utf8_lossy(nvme_ioctl::i8_as_u8(
            &amazon_nvme::AmznVS::from_nvme_id_ctrl(&val).bdev
        ))
        .trim_end()
    );
    Ok(())
}
