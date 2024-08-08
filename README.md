# Fix Your Back remainder in Rust as a debian package and systemd service

## Instalation
Clone the repo
```
git clone https://github.com/felidobrinin/fix-your-back
cd fix-your-back
```
Building and installing
```
cargo deb --install
# upx --best --lzma target/release/fix-your-back # Uncomment this line if you want a smaller binary
```


# TODO

To run `upx --best --lzma binary` on your binary before it's packaged into a `.deb` using `cargo deb`, you can create a custom build script in your `Cargo.toml` that will compress the binary after the release build but before the packaging step.

Here's how you can set it up:

1. **Create a build script**: Create a new file called `build.rs` in the root of your project (same level as `Cargo.toml`).

2. **Edit `build.rs` to include the UPX compression step**:
   ```rust
   use std::process::Command;

   fn main() {
       // This will run the release build
       println!("cargo:rerun-if-changed=src/main.rs");

       // Define the path to the binary
       let binary_path = "target/release/your_binary_name"; // Replace 'your_binary_name' with the actual binary name

       // Run UPX command
       let output = Command::new("upx")
           .args(&["--best", "--lzma", binary_path])
           .output()
           .expect("Failed to execute UPX");

       // Check if the command was successful
       if !output.status.success() {
           panic!(
               "UPX compression failed: {}",
               String::from_utf8_lossy(&output.stderr)
           );
       }

       println!("UPX compression successful");
   }
   ```

3. **Update `Cargo.toml` to use the build script**:
   ```toml
   [package]
   # ... other configurations ...
   build = "build.rs"

   [build-dependencies]
   # ... any build dependencies if needed ...
   ```

4. **Ensure `cargo-deb` uses the compressed binary**: `cargo-deb` should automatically use the binary produced by the build process, so no additional configuration should be necessary if you set the paths correctly.

5. **Run the build and packaging commands**:
   ```sh
   cargo deb
   ```

When you run `cargo deb`, the custom build script will be executed, compressing the binary with UPX before it is packaged into the `.deb` file.

Ensure that `upx` is installed and available in your `PATH`. You can install it on Ubuntu-based systems with:
```sh
sudo apt-get install upx-ucl
```
