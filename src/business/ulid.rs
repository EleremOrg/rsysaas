use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Ulid {
    last_timestamp: u64,
    sequence: u16,
}

impl Ulid {
    pub fn new() -> Self {
        Ulid {
            last_timestamp: 0,
            sequence: 0,
        }
    }

    pub async fn generate(
        &mut self,
        request_id: &u32,
        customer_id: &u32,
        main_item_id: &u32,
        entity_id: &u32,
    ) -> String {
        let mut ulid_bytes = [0u8; 16];

        // Current timestamp since UNIX epoch in milliseconds
        let current_timestamp = Self::get_current_timestamp().await;

        // Ensure the timestamp is never the same as the previous one
        self.last_timestamp = std::cmp::max(self.last_timestamp, current_timestamp);

        // Pack the first 6 bytes with the timestamp (48 bits)
        for i in (0..6).rev() {
            ulid_bytes[i] = (self.last_timestamp & 0xFF) as u8;
            self.last_timestamp >>= 8;
        }

        let hash_result =
            Self::get_random_portion(request_id, customer_id, main_item_id, entity_id).await;

        for i in 0..8 {
            ulid_bytes[i + 6] = ((hash_result >> (8 * (7 - i))) & 0xFF) as u8;
        }

        // Increment the sequence to avoid collisions within the same millisecond
        self.sequence = self.sequence.wrapping_add(1);

        // Pack the last 2 bytes with the sequence (16 bits)
        ulid_bytes[14] = (self.sequence >> 8) as u8;
        ulid_bytes[15] = self.sequence as u8;

        Self::bytes_to_string(ulid_bytes).await
    }

    async fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64
    }

    async fn get_random_portion(
        request_id: &u32,
        customer_id: &u32,
        main_item_id: &u32,
        entity_id: &u32,
    ) -> u64 {
        // Generate the random portion using the hash of the fields
        let mut hasher = DefaultHasher::new();
        request_id.hash(&mut hasher);
        customer_id.hash(&mut hasher);
        main_item_id.hash(&mut hasher);
        entity_id.hash(&mut hasher);
        hasher.finish()
    }

    async fn bytes_to_string(ulid_bytes: [u8; 16]) -> String {
        // Convert bytes to a string representation
        ulid_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}
