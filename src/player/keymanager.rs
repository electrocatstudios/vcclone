pub struct KeyManager {
    keys_down: Vec::<String>
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            keys_down: Vec::new()
        }
    }

    pub fn handle_key_down(&mut self, key: String) {
        if !self.keys_down.contains(&key) {
            self.keys_down.push(key);
        }
    }

    pub fn handle_key_up(&mut self, key: String) {
        self.keys_down.retain(|k| k != &key);
    }

    pub fn is_key_down(&self, key: &str) -> bool {
        self.keys_down.contains(&key.to_string())
    }
}