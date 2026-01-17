use tokio::io::{AsyncReadExt};
use std::io;

use zstd;

// 变长前缀编码：LEB128 风格
fn write_varint(mut n: usize, out: &mut Vec<u8>) {
    while n >= 0x80 {
        out.push((n as u8) | 0x80);
        n >>= 7;
    }
    out.push(n as u8);
}

async fn read_varint<R: AsyncReadExt + Unpin>(r: &mut R) -> io::Result<usize> {
    let mut shift = 0;
    let mut result: usize = 0;
    loop {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf).await?;
        let byte = buf[0] as usize;
        result |= (byte & 0x7F) << shift;
        if (byte & 0x80) == 0 {
            return Ok(result);
        }
        shift += 7;
        if shift >= 64 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "varint too long"));
        }
    }
}

// frame: [varint(len(compressed_payload))][compressed_payload]
pub async fn read_frame(stream: &mut (impl AsyncReadExt + Unpin)) -> io::Result<Vec<u8>> {
    let comp_len = read_varint(stream).await?;
    let mut comp = vec![0u8; comp_len];
    stream.read_exact(&mut comp).await?;
    // 解压
    let payload = zstd::decode_all(&comp[..]).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(payload)
}

pub fn frame(payload: &[u8]) -> io::Result<Vec<u8>> {
    // 压缩
    let compressed = zstd::encode_all(payload, 0).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    // 前缀长度（变长编码）
    let mut prefix = Vec::new();
    write_varint(compressed.len(), &mut prefix);
    let mut out = Vec::with_capacity(prefix.len() + compressed.len());
    out.extend_from_slice(&prefix);
    out.extend_from_slice(&compressed);
    Ok(out)
}
