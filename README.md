# Non-blocking Read and Write a Linux File Descriptor

## example

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
