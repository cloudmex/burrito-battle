use crate::*;
 
#[near_bindgen]
impl Contract {

    pub fn add_whitelist(&mut self, address_contract: AccountId, contract_name: String) -> String {
        assert_eq!(address_contract.to_string().is_empty(),false,"EL contrato no debe estar vacio");
        assert_eq!(contract_name.is_empty(), false, "El nombre del contrato no debe estar vacio");

        let contract_exist = self.whitelist_contracts.get(&address_contract.clone());

        if !contract_exist.is_none() {
            assert_eq!(
                contract_exist.unwrap().contract_name.is_empty(),
                true,
                "El contrato ya se encuentra registrado en el whitelist"
            );
        }

        let new_ext_contract = ExternalContract {
            register_address: env::signer_account_id(),
            contract_name: contract_name.clone(),
        };
    
        self.whitelist_contracts.insert(&address_contract.clone(), &new_ext_contract);

        "Contrato agregado al whitelist con éxito".to_string()    
    }

    pub fn is_white_listed(&self) -> String  {
        let contract_exist = self.whitelist_contracts.get(&env::predecessor_account_id());

        if !contract_exist.is_none() {
            return "No te encuentras registrado en el whitelist".to_string();
        }

        return "Te encuentras registrado en el whitelist".to_string();;

    }

    pub fn assert_whitelist(&self, account_id: AccountId) {
        let contract_exist = self.whitelist_contracts.get(&env::predecessor_account_id());

        if contract_exist.is_none() {
            env::panic_str("No te encuentras registrado en el whitelist");
        }
    }

}
