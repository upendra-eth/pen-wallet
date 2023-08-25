#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp37 {
	use ink::{
        codegen::{EmitEvent, Env},
        prelude::vec::Vec,
    };
    
    // imports from openbrush
	use openbrush::traits::Storage;
	use openbrush::contracts::psp37::extensions::batch::*;
	use openbrush::contracts::psp37::extensions::burnable::*;
	use openbrush::contracts::psp37::extensions::mintable::*;
	use openbrush::contracts::psp37::extensions::metadata::*;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp37: psp37::Data,
		#[storage_field]
		metadata: metadata::Data,
    }


    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        _from: Option<AccountId>,
        #[ink(topic)]
        _to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
        #[ink(topic)]
        _amount: Balance,
    }

    /// Event emitted when a token Batch transfer occurs.
    #[ink(event)]
    pub struct TransferBatch {
        #[ink(topic)]
        _from: Option<AccountId>,
        #[ink(topic)]
        _to: Option<AccountId>,
        #[ink(topic)]
        _ids_amounts: Vec<(Id, Balance)>,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        _owner: AccountId,
        #[ink(topic)]
        _operator: AccountId,
        #[ink(topic)]
        _id: Option<Id>,
        #[ink(topic)]
        _value: Balance,
    }
    
    // Section contains default implementation without any modifications
	impl PSP37 for Contract {}
	impl PSP37Batch for Contract {}
	impl PSP37Burnable for Contract {}
	impl PSP37Mintable for Contract {}
	impl PSP37Metadata for Contract {}

	    
    impl psp37::Internal for Contract {
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            id: Id,
            _amount: Balance,
        ) {
            self.env().emit_event(Transfer {
                _from,
                _to,
                id,
                _amount,
            });
        }
        fn _emit_transfer_batch_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _ids_amounts: Vec<(Id, Balance)>,
        ) {
            self.env().emit_event(TransferBatch {
                _from,
                _to,
                _ids_amounts,
            });
        }
        fn _emit_approval_event(
            &self,
            _owner: AccountId,
            _operator: AccountId,
            _id: Option<Id>,
            _value: Balance,
        ) {
            self.env().emit_event(Approval {
                _owner,
                _operator,
                _id,
                _value,
            });
        }
    }


     
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}