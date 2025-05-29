use tokio::io::simplex;

#[tokio::test]
async fn handshake() {
    let (mut sr, mut cw) = simplex(u16::MAX as usize);
    let (mut cr, mut sw) = simplex(u16::MAX as usize);

    let (client_handshake, server_handshake) = tokio::join!(
        client::connection::do_handshake(&mut cr, &mut cw),
        server::connection::do_handshake(&mut sr, &mut sw),
    );

    client_handshake.unwrap();
    server_handshake.unwrap();
}
