extern crate platform;
use platform::MsgResult;
use platform::Platform;

pub fn init<K>(_platform: &mut Platform<K>) -> MsgResult<()> {
    // do nothing
    Ok(())
}
