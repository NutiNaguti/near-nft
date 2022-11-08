use near_contract_standards::non_fungible_token::{
    metadata::{NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC},
    NonFungibleToken, Token, TokenId,
};
use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    collections::LazyOption,
    *,
};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&NFTContractMetadata {
                    spec: NFT_METADATA_SPEC.to_string(),
                    name: "NUTI TOKEN".to_string(),
                    symbol: "NUTI".to_string(),
                    icon: Some(
                        "https://raw.githubusercontent.com/Gowee/nyancat-svg/main/nyancat.svg"
                            .to_string(),
                    ),
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                }),
            ),
        }
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receive_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.token
            .internal_mint(token_id, receive_id, Some(token_metadata))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
