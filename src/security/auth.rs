
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

pub struct SecurityConfig {
    jwt_secret: String,
    token_expiration: Duration,
}

impl SecurityConfig {
    pub fn generate_token(&self, agent_id: &str) -> Result<String, Error> {
        let claims = Claims {
            agent_id: agent_id.to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize + self.token_expiration.as_secs() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes())
        )
    }
}