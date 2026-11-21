use std::io::{self, Write};

use axum::{
    body::Bytes,
    http::{HeaderMap, header::ACCEPT_ENCODING},
};
use flate2::{Compression, write::GzEncoder};

const BROTLI_BUFFER_SIZE: usize = 4096;
const BROTLI_QUALITY: u32 = 11;
const BROTLI_WINDOW_SIZE: u32 = 22;

pub(crate) struct Encoded {
    identity: Bytes,
    gzip: Bytes,
    brotli: Bytes,
}

pub(crate) struct Negotiated {
    pub(crate) body: Bytes,
    pub(crate) encoding: Option<&'static str>,
}

impl Encoded {
    pub(super) fn new(body: Vec<u8>) -> io::Result<Self> {
        let mut gzip = GzEncoder::new(Vec::new(), Compression::best());
        gzip.write_all(&body)?;
        let mut brotli = brotli::CompressorWriter::new(
            Vec::new(),
            BROTLI_BUFFER_SIZE,
            BROTLI_QUALITY,
            BROTLI_WINDOW_SIZE,
        );
        brotli.write_all(&body)?;
        Ok(Self {
            gzip: gzip.finish()?.into(),
            brotli: brotli.into_inner().into(),
            identity: body.into(),
        })
    }

    pub(crate) fn negotiate(&self, headers: &HeaderMap) -> Negotiated {
        let accepted = headers
            .get(ACCEPT_ENCODING)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        if accepts(accepted, "br") {
            Negotiated {
                body: self.brotli.clone(),
                encoding: Some("br"),
            }
        } else if accepts(accepted, "gzip") {
            Negotiated {
                body: self.gzip.clone(),
                encoding: Some("gzip"),
            }
        } else {
            Negotiated {
                body: self.identity.clone(),
                encoding: None,
            }
        }
    }
}

fn accepts(accept_encoding: &str, encoding: &str) -> bool {
    accept_encoding.split(',').any(|token| {
        token
            .split(';')
            .next()
            .is_some_and(|name| name.trim() == encoding)
    })
}
