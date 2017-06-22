Host: Thinkpad X220 (Intel i5) running Debian
Target: Raspberry Pi Model 3 B running Raspbian Lite

Host triple:    x86_64-unknown-linux-gnu

Target triple:  arm-unknown-linux-gnueabi
or maybe:       armv7-unknown-linux-gnueabihf

I can't tell which is correct, former will be missing some armv7 optimizations but might have better compatibility?


Following the guide from https://github.com/japaric/rust-cross

From the host system:

`sudo apt-get install -qq gcc-arm-linux-gnueabihf`

`rustup target add armv7-unknown-linux-gnueabihf`

create file ~/.cargo/config with these contents:
```
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

Now you can compile with: `cargo build --target=armv7-unknown-linux-gnueabihf`

