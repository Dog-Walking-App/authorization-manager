use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, errors::Error, TokenData};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json;

pub struct JWT {
    secret: String,
}

impl JWT {
    pub fn new(secret: String) -> JWT {
        JWT { secret }
    }
    
    pub fn generate<T: Serialize>(&self, claims: &T) -> String {
      let header = Header::default();
      let key = EncodingKey::from_secret(self.secret.as_ref());
      let token = encode(&header, &claims, &key).unwrap();
      token
    }

    fn decode<T: for<'de> Deserialize<'de> + DeserializeOwned>(
        &self,
        value: &str,
    ) -> Result<TokenData<T>, Error> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_ref());
        let validation = Validation::default();
        decode::<T>(value, &decoding_key, &validation)
    }

    pub fn get_claims<T: for<'de> Deserialize<'de> + DeserializeOwned>(
        &self,
        value: &str,
    ) -> Result<T, Error> {
        let claims = self.decode::<T>(value)?
            .claims;
        
        Ok(claims)
    }

    pub fn validate(&self, token: &str) -> Result<(), Error> {
        self.decode::<serde_json::Value>(token)?;
        Ok(())
    }
}
