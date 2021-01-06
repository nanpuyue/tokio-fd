# Non-blocking Read and Write a Linux/Unix File Descriptor

[![Crates.io](https://img.shields.io/crates/v/tokio-fd?color=green)](https://crates.io/crates/tokio-fd)

## Example

```rust
use std::convert::TryFrom;
use std::io::Result;

use tokio::prelude::*;
use tokio_fd::AsyncFd;

#[tokio::main]
async fn main() -> Result<()> {
    let mut stdin = AsyncFd::try_from(libc::STDIN_FILENO)?;
    let mut stdout = AsyncFd::try_from(libc::STDOUT_FILENO)?;
    let mut buf = vec![0; 1024];

    while let Ok(n) = stdin.read(&mut buf).await {
        stdout.write(&buf[..n]).await?;
    }
    Ok(())
}
```

## License

This project is licensed under either of

* [Apache License, Version 2.0](https://github.com/nanpuyue/tokio-fd/blob/master/LICENSE-APACHE)
* [MIT License](https://github.com/nanpuyue/tokio-fd/blob/master/LICENSE-MIT)

at your option.
