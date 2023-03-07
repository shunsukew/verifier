#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod verifier {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Verifier {
    }

    impl Verifier {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { }
        }


        #[ink(message)]
        pub fn verify(&self, data: String, signer: AccountId, signature: [u8; 65]) -> bool {
            ink::env::debug_println!("data {:?}", data);
            ink::env::debug_println!("signer {:?}", signer);
            ink::env::debug_println!("signature {:?}", signature);

            let mut message_hash = <ink::env::hash::Blake2x256 as ink::env::hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<ink::env::hash::Blake2x256>(&data.as_bytes(), &mut message_hash);

            ink::env::debug_println!("message_hash {:?}", message_hash);

            let output = self.env().ecdsa_recover(&signature, &message_hash).expect("Failed to recover");

            ink::env::debug_println!("pubkey {:?}", output);

            let mut signature_account_id = <ink::env::hash::Blake2x256 as ink::env::hash::HashOutput>::Type::default();
            ink::env::hash_encoded::<ink::env::hash::Blake2x256, _>(&output, &mut signature_account_id);

            ink::env::debug_println!("Sig account id {:?}", AccountId::from(signature_account_id));

            signer == AccountId::from(signature_account_id)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::default_accounts;

        #[ink::test]
        fn verify_works() {
            let verifier = Verifier::new();

            let data = 10;
            let signer = default_accounts().alice;
            
        }
    }
}
