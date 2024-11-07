use std::io::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::v5::{client::Greeting, server::Choice};

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    /// Reads a greeting from the stream and converts it into a Greeting struct.
    /// Greating is expected to be in the format defined by the SOCKS5 protocol.
    pub async fn read_greeting(&mut self, buffer: &mut [u8]) -> Result<Greeting, Error> {
        let size = self.stream.read(buffer).await?;
        if size == 0 {
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "stream closed",
            ));
        }

        Ok(Greeting::from(&buffer[..size]))
    }

    /// Writes a choice to the stream.
    /// The choice is expected to be in the format defined by the SOCKS5 protocol.
    pub async fn write_choice(&mut self, choice: Choice) -> Result<(), Error> {
        let choice_buffer: [u8; 2] = choice.into();
        self.stream.write(&choice_buffer).await?;

        Ok(())
    }

    /// Reads a request from the stream and converts it into the specified type R.
    pub async fn read_request<R: From<Vec<u8>>>(
        &mut self,
        mut buffer: Vec<u8>,
    ) -> Result<R, Error> {
        let size = self.stream.read(&mut buffer).await?;
        if size == 0 {
            return Err(Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "stream closed",
            ));
        }
        Ok(R::from(buffer[..size].to_vec()))
    }

    // /// Writes a response to the stream.
    pub async fn write_response<R: Into<Vec<u8>>>(&mut self, response: R) -> Result<(), Error> {
        let response_buffer: Vec<u8> = response.into();
        self.stream.write(&response_buffer).await?;
        Ok(())
    }
}

impl Into<TcpStream> for Connection {
    fn into(self) -> TcpStream {
        self.stream
    }
}
