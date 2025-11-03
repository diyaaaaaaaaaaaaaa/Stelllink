#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Map
};

// Storage keys
const LINKS: Symbol = symbol_short!("LINKS");
const OWNERS: Symbol = symbol_short!("OWNERS");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkRecord {
    pub destination_url: String,
    pub created_ledger: u32,
    pub owner: Address,
}

#[contract]
pub struct StellarLinkContract;

#[contractimpl]
impl StellarLinkContract {
    /// Creates a new short link
    /// Returns the short_key that was created
    pub fn create_link(
        env: Env,
        owner: Address,
        long_url: String,
        custom_key: Option<String>,
    ) -> String {
        // Authenticate the owner
        owner.require_auth();

        // Validate URL is not empty
        if long_url.len() == 0 {
            panic!("URL cannot be empty");
        }

        // Determine the short key
        let short_key = match custom_key {
            Some(key) => {
                // Validate custom key
                if key.len() == 0 || key.len() > 64 {
                    panic!("Custom key must be between 1 and 64 characters");
                }
                key
            }
            None => {
                // Generate a random key
                Self::generate_random_key(&env)
            }
        };

        // Check if key already exists
        let mut links: Map<String, LinkRecord> = env
            .storage()
            .persistent()
            .get(&LINKS)
            .unwrap_or(Map::new(&env));

        if links.contains_key(short_key.clone()) {
            panic!("Link key already exists");
        }

        // Create the link record
        let link_record = LinkRecord {
            destination_url: long_url,
            created_ledger: env.ledger().sequence(),
            owner: owner.clone(),
        };

        // Store the link
        links.set(short_key.clone(), link_record);
        env.storage().persistent().set(&LINKS, &links);

        // Store the owner mapping
        let mut owners: Map<String, Address> = env
            .storage()
            .persistent()
            .get(&OWNERS)
            .unwrap_or(Map::new(&env));
        
        owners.set(short_key.clone(), owner);
        env.storage().persistent().set(&OWNERS, &owners);

        // Extend TTL for the storage entries
        env.storage().persistent().extend_ttl(&LINKS, 31536000, 31536000); // 1 year
        env.storage().persistent().extend_ttl(&OWNERS, 31536000, 31536000);

        short_key
    }

    /// Updates the destination URL of an existing link
    pub fn update_link(env: Env, owner: Address, short_key: String, new_long_url: String) {
        // Authenticate the owner
        owner.require_auth();

        // Validate URL is not empty
        if new_long_url.len() == 0 {
            panic!("URL cannot be empty");
        }

        // Check ownership
        let owners: Map<String, Address> = env
            .storage()
            .persistent()
            .get(&OWNERS)
            .unwrap_or(Map::new(&env));

        let link_owner = owners.get(short_key.clone()).unwrap_or_else(|| {
            panic!("Link not found");
        });

        if link_owner != owner {
            panic!("Not the owner of this link");
        }

        // Get links map
        let mut links: Map<String, LinkRecord> = env
            .storage()
            .persistent()
            .get(&LINKS)
            .unwrap_or(Map::new(&env));

        // Get existing link record
        let mut link_record = links.get(short_key.clone()).unwrap_or_else(|| {
            panic!("Link not found");
        });

        // Update the destination URL
        link_record.destination_url = new_long_url;

        // Save the updated record
        links.set(short_key.clone(), link_record);
        env.storage().persistent().set(&LINKS, &links);

        // Extend TTL
        env.storage().persistent().extend_ttl(&LINKS, 31536000, 31536000);
    }

    /// Deletes a link (only owner can delete)
    pub fn delete_link(env: Env, owner: Address, short_key: String) {
        // Authenticate the owner
        owner.require_auth();

        // Check ownership
        let mut owners: Map<String, Address> = env
            .storage()
            .persistent()
            .get(&OWNERS)
            .unwrap_or(Map::new(&env));

        let link_owner = owners.get(short_key.clone()).unwrap_or_else(|| {
            panic!("Link not found");
        });

        if link_owner != owner {
            panic!("Not the owner of this link");
        }

        // Remove from links map
        let mut links: Map<String, LinkRecord> = env
            .storage()
            .persistent()
            .get(&LINKS)
            .unwrap_or(Map::new(&env));

        links.remove(short_key.clone());
        env.storage().persistent().set(&LINKS, &links);

        // Remove from owners map
        owners.remove(short_key);
        env.storage().persistent().set(&OWNERS, &owners);
    }

    /// Gets the destination URL for a short key (public, read-only)
    pub fn get_url(env: Env, short_key: String) -> String {
        let links: Map<String, LinkRecord> = env
            .storage()
            .persistent()
            .get(&LINKS)
            .unwrap_or(Map::new(&env));

        let link_record = links.get(short_key).unwrap_or_else(|| {
            panic!("Link not found");
        });

        link_record.destination_url
    }

    /// Gets the full link record (for debugging/frontend)
    pub fn get_link(env: Env, short_key: String) -> LinkRecord {
        let links: Map<String, LinkRecord> = env
            .storage()
            .persistent()
            .get(&LINKS)
            .unwrap_or(Map::new(&env));

        links.get(short_key).unwrap_or_else(|| {
            panic!("Link not found");
        })
    }

    /// Gets the owner of a link
    pub fn get_owner(env: Env, short_key: String) -> Address {
        let owners: Map<String, Address> = env
            .storage()
            .persistent()
            .get(&OWNERS)
            .unwrap_or(Map::new(&env));

        owners.get(short_key).unwrap_or_else(|| {
            panic!("Link not found");
        })
    }

    // Helper function to generate a random short key
    fn generate_random_key(env: &Env) -> String {
        // Use ledger sequence and timestamp for pseudo-randomness
        let ledger_seq = env.ledger().sequence();
        let timestamp = env.ledger().timestamp();
        
        // Combine and hash to create a key
        let combined = (ledger_seq as u64).wrapping_add(timestamp);
        
        // Create a simple base62-like encoding
        let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut key_bytes: [u8; 7] = [0; 7];
        let mut num = combined;
        
        for i in 0..7 {
            let idx = (num % 62) as usize;
            key_bytes[i] = chars[idx];
            num = num / 62;
        }
        
        String::from_bytes(env, &key_bytes)
    }
}

#[cfg(test)]
mod test;