## Status
Development of Dehset VPN in this repository has been discontinued.
A complete protocol rewrite is underway in a new repository.


# DEHSeT VPN - Dynamic Encrypted High-Security Tunnel
Dehset VPN is a proprietary virtual network software designed for tunneling
license server, SSH, and RDP ports. The source code policy is subject to change
and may be open-sourced in the future.

The current version uses symetric encryption. Development of a custom TLS
protocol is currently in progress.

The scope of Dehset VPN is limited to port tunneling. It does not create a full
network or virtual network interface.


## Building
To build this project, make sure OpenSSL is properly installed. Refer to the
[OpenSSL crate documentation](https://docs.rs/openssl/latest/openssl/index.html#building)
for platform-specific installation steps.
