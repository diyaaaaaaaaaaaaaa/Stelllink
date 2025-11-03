#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_create_link_with_custom_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/very/long/url");
    let custom_key = String::from_str(&env, "mylink");

    // Mock the auth so we don't need to sign
    env.mock_all_auths();

    let result = client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    assert_eq!(result, custom_key);

    // Verify the link can be retrieved
    let retrieved_url = client.get_url(&custom_key);
    assert_eq!(retrieved_url, long_url);
}

#[test]
fn test_create_link_without_custom_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/another/url");

    env.mock_all_auths();

    let short_key = client.create_link(&owner, &long_url, &None);

    // Verify a key was generated
    assert!(short_key.len() > 0);

    // Verify the link can be retrieved
    let retrieved_url = client.get_url(&short_key);
    assert_eq!(retrieved_url, long_url);
}

#[test]
#[should_panic(expected = "Link key already exists")]
fn test_create_duplicate_link() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/url1");
    let custom_key = String::from_str(&env, "duplicate");

    env.mock_all_auths();

    // Create first link
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Try to create duplicate - should panic
    let long_url2 = String::from_str(&env, "https://example.com/url2");
    client.create_link(&owner, &long_url2, &Some(custom_key));
}

#[test]
fn test_update_link() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/original");
    let custom_key = String::from_str(&env, "updateme");

    env.mock_all_auths();

    // Create link
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Update the link
    let new_url = String::from_str(&env, "https://example.com/updated");
    client.update_link(&owner, &custom_key, &new_url);

    // Verify the URL was updated
    let retrieved_url = client.get_url(&custom_key);
    assert_eq!(retrieved_url, new_url);
}

#[test]
#[should_panic(expected = "Not the owner of this link")]
fn test_update_link_wrong_owner() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let other_user = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/original");
    let custom_key = String::from_str(&env, "mylink");

    env.mock_all_auths();

    // Create link as owner
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Try to update as different user - should panic
    let new_url = String::from_str(&env, "https://example.com/hacked");
    client.update_link(&other_user, &custom_key, &new_url);
}

#[test]
fn test_delete_link() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/deleteme");
    let custom_key = String::from_str(&env, "deletethis");

    env.mock_all_auths();

    // Create link
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Delete the link
    client.delete_link(&owner, &custom_key);

    // Verify the link no longer exists - should panic when trying to get it
    let result = std::panic::catch_unwind(|| {
        client.get_url(&custom_key);
    });
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "Not the owner of this link")]
fn test_delete_link_wrong_owner() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let other_user = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/protected");
    let custom_key = String::from_str(&env, "protected");

    env.mock_all_auths();

    // Create link as owner
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Try to delete as different user - should panic
    client.delete_link(&other_user, &custom_key);
}

#[test]
fn test_get_link_record() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/test");
    let custom_key = String::from_str(&env, "testlink");

    env.mock_all_auths();

    // Create link
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Get full link record
    let link_record = client.get_link(&custom_key);

    assert_eq!(link_record.destination_url, long_url);
    assert_eq!(link_record.owner, owner);
    assert!(link_record.created_ledger > 0);
}

#[test]
fn test_get_owner() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/owned");
    let custom_key = String::from_str(&env, "owned");

    env.mock_all_auths();

    // Create link
    client.create_link(&owner, &long_url, &Some(custom_key.clone()));

    // Verify owner
    let retrieved_owner = client.get_owner(&custom_key);
    assert_eq!(retrieved_owner, owner);
}

#[test]
#[should_panic(expected = "Link not found")]
fn test_get_nonexistent_link() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let nonexistent_key = String::from_str(&env, "doesnotexist");
    
    // Should panic when trying to get a link that doesn't exist
    client.get_url(&nonexistent_key);
}

#[test]
#[should_panic(expected = "URL cannot be empty")]
fn test_create_link_empty_url() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let empty_url = String::from_str(&env, "");
    let custom_key = String::from_str(&env, "emptyurl");

    env.mock_all_auths();

    // Should panic with empty URL
    client.create_link(&owner, &empty_url, &Some(custom_key));
}

#[test]
#[should_panic(expected = "Custom key must be between 1 and 64 characters")]
fn test_create_link_empty_custom_key() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let long_url = String::from_str(&env, "https://example.com/test");
    let empty_key = String::from_str(&env, "");

    env.mock_all_auths();

    // Should panic with empty custom key
    client.create_link(&owner, &long_url, &Some(empty_key));
}

#[test]
fn test_multiple_users_different_links() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    let url1 = String::from_str(&env, "https://example.com/user1");
    let url2 = String::from_str(&env, "https://example.com/user2");
    
    let key1 = String::from_str(&env, "user1link");
    let key2 = String::from_str(&env, "user2link");

    env.mock_all_auths();

    // Both users create their own links
    client.create_link(&user1, &url1, &Some(key1.clone()));
    client.create_link(&user2, &url2, &Some(key2.clone()));

    // Verify both links exist and are correct
    assert_eq!(client.get_url(&key1), url1);
    assert_eq!(client.get_url(&key2), url2);

    // Verify ownership
    assert_eq!(client.get_owner(&key1), user1);
    assert_eq!(client.get_owner(&key2), user2);
}

#[test]
fn test_update_link_preserves_metadata() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StellarLinkContract);
    let client = StellarLinkContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let original_url = String::from_str(&env, "https://example.com/original");
    let custom_key = String::from_str(&env, "metadata");

    env.mock_all_auths();

    // Create link
    client.create_link(&owner, &original_url, &Some(custom_key.clone()));

    // Get original metadata
    let original_record = client.get_link(&custom_key);
    let original_ledger = original_record.created_ledger;

    // Update URL
    let new_url = String::from_str(&env, "https://example.com/updated");
    client.update_link(&owner, &custom_key, &new_url);

    // Get updated record
    let updated_record = client.get_link(&custom_key);

    // Verify URL changed but metadata preserved
    assert_eq!(updated_record.destination_url, new_url);
    assert_eq!(updated_record.owner, owner);
    assert_eq!(updated_record.created_ledger, original_ledger);
}