use base64::Engine;
use magic_crypt::generic_array::typenum::U256;
use magic_crypt::MagicCryptTrait;
use rand::Rng;
use std::io::Cursor;

pub fn create_password_hash(password: String, hash_key: String) -> String {
    let mc = magic_crypt::new_magic_crypt!(hash_key, 256);

    let mut reader = Cursor::new(password);
    let mut writer = Vec::new();
    mc.encrypt_reader_to_writer2::<U256>(&mut reader, &mut writer)
        .unwrap();
    let encrypted = base64::engine::general_purpose::STANDARD.encode(&writer);

    encrypted
}

async fn generate_jwt_session_id(_user_id: u16) -> String {
    let mut rand = rand::thread_rng();
    let temp_new_session_id: u128 = rand.gen();
    temp_new_session_id.to_string()
}
