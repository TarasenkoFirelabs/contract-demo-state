pub type ActionId = String;
pub type PermissionId = String;
pub type ContractId = ContractId;

pub struct ContractPermission {
    permission_id:PermissionId,
    contract_id:
    action_id:ActionId,
}

pub trait ContractAutorize{
    fn is_allowed(contract_id:ContractId)->bool;
    fn grant(contract_id:ContractId);
    fn grant(contract_id:ContractId,action_id:ActionId);
    fn grant_all();
}

pub struct ContractAuthorization {
    allowed_contracts: UnorderedMap<PermissionId,ContractPermission>,
}