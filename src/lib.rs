use near_contract_standards::non_fungible_token::{
    approval::NonFungibleTokenApproval,
    core::NonFungibleTokenCore,
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

    pub fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.token.nft_token(token_id)
    }

    #[payable]
    pub fn nft_approve(&mut self, token_id: TokenId, account_id: AccountId, msg: Option<String>) {
        self.token.nft_approve(token_id, account_id, msg);
    }

    pub fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) {
        self.token
            .nft_is_approved(token_id, approved_account_id, approval_id);
    }

    pub fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        self.token.nft_revoke(token_id, account_id);
    }

    pub fn nft_revoke_all(&mut self, token_id: TokenId) {
        self.token.nft_revoke_all(token_id);
    }

    #[payable]
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        self.token
            .nft_transfer(receiver_id, token_id, approval_id, memo)
    }
}
