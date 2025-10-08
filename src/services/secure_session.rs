use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
use std::sync::{Mutex, LazyLock};

/// Generatore sicuro di session ID
pub struct SecureSessionIdGenerator {
    rng: StdRng,
    counter: u64,
}

impl SecureSessionIdGenerator {
    /// Crea un nuovo generatore con seed sicuro
    pub fn new() -> Self {
        // Combina multiple fonti di entropia
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        let process_id = process::id() as u64;
        
        // Seed da fonti di entropia del sistema
        let mut hasher = Sha256::new();
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&process_id.to_le_bytes());
        
        // Aggiungi entropia aggiuntiva dal sistema
        let mut system_entropy = [0u8; 32];
        getrandom::getrandom(&mut system_entropy)
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
    
    /// Genera un session ID sicuro
    pub fn generate(&mut self) -> String {
        // Combina timestamp ad alta risoluzione
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        // Counter per evitare collisioni
        self.counter = self.counter.wrapping_add(1);
        
        // 32 bytes di entropia random
        let mut random_bytes = [0u8; 32];
        self.rng.fill(&mut random_bytes);
        
        // Hash finale per rendere imprevedibile la relazione tra input e output
        let mut hasher = Sha256::new();
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(&self.counter.to_le_bytes());
        hasher.update(&random_bytes);
        
        let result = hasher.finalize();
        
        // Ritorna come stringa esadecimale (64 caratteri)
        hex::encode(result)
    }
}

/// Istanza globale thread-safe del generatore
static GENERATOR: LazyLock<Mutex<SecureSessionIdGenerator>> = LazyLock::new(|| {
    Mutex::new(SecureSessionIdGenerator::new())
});

/// Genera un session ID sicuro usando il generatore globale
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
        
        // Genera 1000 ID e verifica che siano tutti unici
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
        
        // Verifica formato esadecimale
        assert_eq!(id.len(), 64);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_session_id_entropy() {
        let mut generator = SecureSessionIdGenerator::new();
        let id1 = generator.generate();
        let id2 = generator.generate();
        
        // Gli ID devono essere diversi
        assert_ne!(id1, id2);
        
        // Verifica che abbiano alta entropia (Hamming distance alta)
        let diff_chars: usize = id1.chars()
            .zip(id2.chars())
            .map(|(a, b)| if a != b { 1 } else { 0 })
            .sum();
        
        // Almeno il 25% dei caratteri deve essere diverso
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
        // Test che il session ID sia crittograficamente sicuro
        let id = generate_secure_session_id();
        
        // Verifica lunghezza (SHA256 = 32 bytes = 64 caratteri hex)
        assert_eq!(id.len(), 64);
        
        // Verifica che sia solo caratteri esadecimali
        assert!(id.chars().all(|c| "0123456789abcdef".contains(c)));
        
        // Verifica che non contenga pattern ovvi
        assert!(!id.contains("0000000000"));
        assert!(!id.contains("ffffffffff"));
        assert!(!id.starts_with("000000"));
        assert!(!id.ends_with("000000"));
    }
}