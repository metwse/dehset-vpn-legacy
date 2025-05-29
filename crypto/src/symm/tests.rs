use super::*;
use crate::DynResult;
use proto_core::random_bytes;

#[test]
fn random() -> DynResult<()> {
    let aes128_cbc = Aes128CbcSha256::new(Vec::from(random_bytes!(16)));

    let data = random_bytes!(1024);
    let payload = aes128_cbc.encrypt(None, &data)?;

    assert_eq!(data, &aes128_cbc.decrypt(None, &payload)?[..]);

    Ok(())
}
