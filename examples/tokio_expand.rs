use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

// to check expand result
// cargo expand --example tokio_expand

/*

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        loop {
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            {
                                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                                    &["failed to read from socket; err = ", "\n"],
                                    &[::core::fmt::ArgumentV1::new(&e, ::core::fmt::Debug::fmt)],
                                ));
                            };
                            return;
                        }
                    };
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        {
                            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                                &["failed to write to socket; err = ", "\n"],
                                &[::core::fmt::ArgumentV1::new(&e, ::core::fmt::Debug::fmt)],
                            ));
                        };
                        return;
                    }
                }
            });
        }
    };
    #[allow(clippy::expect_used)]
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
*/