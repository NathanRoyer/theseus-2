// NIC drivers
extern crate e1000;
extern crate mlx5;

// FS drivers
extern crate fat32;
extern crate ext4;

// Timer drivers
extern crate pmu_x86_64;


extern crate platform;

use platform::Lock;
use platform::MsgResult;
pub type Platform = platform::Platform<Kernel>;

pub struct Kernel;

pub fn theseus_init(platform: &mut Platform) -> MsgResult<()> {

    platform.kernel = Lock::new(Box::new(Some(Kernel)));

    // NIC drivers
    e1000::init(platform)?;
    mlx5::init(platform)?;

    // FS drivers
    fat32::init(platform)?;
    ext4::init(platform)?;

    // Timer drivers
    pmu_x86_64::init(platform)?;

    Ok(())
}