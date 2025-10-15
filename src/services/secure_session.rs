use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
use std::sync::{Mutex, LazyLock};

pub struct SecureSessionIdGenerator {
    rng: StdRng,
    counter: u64,
}

impl SecureSessionIdGenerator {
    pub fn new() -> Self {
        // Combina multiple fonti di entropia
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let process_id = process::id() as u64;

        let mut hasher = Sha256::new();
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&process_id.to_le_bytes());

        let mut system_entropy = [0u8; 32];
        getrandom::fill(&mut system_entropy)
            .expect("Failed to get system entropy");
        hasher.update(&system_entropy);
        
        let seed_hash = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&seed_hash[..32]);
        
        let rng = StdRng::from_seed(seed);
        
        Self {
            rng,
            counter: 0,
        }
    }

    pub fn generate(&mut self) -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        // Counter per evitare collisioni
        self.counter = self.counter.wrapping_add(1);

        let mut random_bytes = [0u8; 32];
        self.rng.fill(&mut random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&self.counter.to_le_bytes());
        hasher.update(&random_bytes);
        
        let result = hasher.finalize();

        hex::encode(result)
    }
}

static GENERATOR: LazyLock<Mutex<SecureSessionIdGenerator>> = LazyLock::new(|| {
    Mutex::new(SecureSessionIdGenerator::new())
});

pub fn generate_secure_session_id() -> String {
    let mut generator = GENERATOR.lock().expect("Failed to lock generator");
    generator.generate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    
    #[test]
    fn test_session_id_uniqueness() {
        let mut generator = SecureSessionIdGenerator::new();
        let mut ids = HashSet::new();

        for _ in 0..1000 {
            let id = generator.generate();
            assert_eq!(id.len(), 64); // Lunghezza SHA256 hex
            assert!(ids.insert(id), "Duplicate session ID generated");
        }
    }
    
    #[test]
    fn test_session_id_format() {
        let mut generator = SecureSessionIdGenerator::new();
        let id = generator.generate();

        assert_eq!(id.len(), 64);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_session_id_entropy() {
        let mut generator = SecureSessionIdGenerator::new();
        let id1 = generator.generate();
        let id2 = generator.generate();

        assert_ne!(id1, id2);

        let diff_chars: usize = id1.chars()
            .zip(id2.chars())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum();

        assert!(diff_chars >= 16, "Session IDs have low entropy");
    }
    
    #[test]
    fn test_global_generator() {
        let id1 = generate_secure_session_id();
        let id2 = generate_secure_session_id();
        
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 64);
        assert_eq!(id2.len(), 64);
    }
    
    #[test]
    fn test_session_id_strength() {
        let id = generate_secure_session_id();
        
        assert_eq!(id.len(), 64);
        
        assert!(id.chars().all(|c| "0123456789abcdef".contains(c)));
        
        assert!(!id.contains("0000000000"));
        assert!(!id.contains("ffffffffff"));
        assert!(!id.starts_with("000000"));
        assert!(!id.ends_with("000000"));
    }
}