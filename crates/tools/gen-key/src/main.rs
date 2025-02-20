pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // Ok for tools.

use lib_utils::b64::b64u_encode;
use rand::RngCore;
use uuid::Uuid;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits = 64 bytes
    rand::thread_rng().fill_bytes(&mut key);
    println!("\nGenerated key from rand::thread_rng():\n{key:?}");

    let user_id: i64 = 12343;
    let uid = Uuid::now_v7();
    let mut uid_bytes = uid.as_bytes().to_vec();
    let userd = user_id.to_le_bytes().to_vec();
    uid_bytes.extend(userd);
    let b64u = b64u_encode(uid_bytes);

    println!("\nKey b64u encoded:\n{b64u}");

    Ok(())
}
