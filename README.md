# Fix Your Back remainder in Rust as a debian package and systemd service

## Instalation
Clone the repo
```
git clone https://github.com/felidobrinin/fix-your-back
cd fix-your-back
cargo build --release && cargo deb
sudo dpkg -i target/debian/fix-your-back_0.1.0-1_amd64.deb
```
