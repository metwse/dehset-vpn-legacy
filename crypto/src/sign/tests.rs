use super::*;
use hex;
use testutil::{DynResult, random_bytes};

#[test]
fn signature() -> DynResult<()> {
    let signer = Hs256::try_new(&hex::decode("01598d62e7028e14aa6a4bc148a9f4b4").unwrap())?;

    let signature = signer.sign(&hex::decode("0ceea336b1b2eb9bbd895688a3fc0208").unwrap())?;

    assert_eq!(
        signature,
        hex::decode("db945088ccfb16fae0c4d329f56a7e60387fbb74156b0b6771688c344a843ee5").unwrap()
    );

    Ok(())
}

#[test]
fn random() -> DynResult<()> {
    let signer = Hs256::try_new(&random_bytes!(512))?;

    let mut data = random_bytes!(512);
    let signature = signer.sign(&data)?;

    assert!(signer.verify(&data, &signature)?);

    data[0] += 1;
    assert!(!signer.verify(&data, &signature)?);

    Ok(())
}

#[test]
fn sign_token() -> DynResult<()> {
    let token = testutil::generate_token(1, String::from("Test"), vec![String::from("test")]);
    let signer = Hs256::try_new(&random_bytes!(512))?;

    let signed_token = super::token::sign_token(token, &signer)?;

    assert_eq!(
        signed_token.signature,
        signer.sign(&signed_token.token.encode()?[..])?
    );

    assert!(signer.verify(
        &signed_token.token.encode()?[..],
        &signed_token.signature[..]
    )?);

    Ok(())
}
