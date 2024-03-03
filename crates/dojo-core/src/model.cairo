use starknet::SyscallResult;

trait Model<T> {
    fn name_hash(self: @T) -> felt252;
    fn keys(self: @T) -> Span<felt252>;
    fn values(self: @T) -> Span<felt252>;
    fn layout(self: @T) -> Span<u8>;
    fn packed_size(self: @T) -> usize;
}

#[starknet::interface]
trait IModel<T> {
    fn name_hash(self: @T) -> felt252;
    fn layout(self: @T) -> Span<felt252>;
    fn schema(self: @T) -> Span<dojo::database::introspect::Member>;
}

#[starknet::interface]
trait IDojoModel<T> {
    fn name_hash(self: @T) -> felt252;
    fn unpacked_size(self: @T) -> usize;
    fn packed_size(self: @T) -> usize;
    fn layout(self: @T) -> Span<u8>;
    fn schema(self: @T) -> dojo::database::introspect::Ty;
}

/// Deploys a model with the given [`ClassHash`] and retrieves it's name.
/// Currently, the model is expected to already be declared by `sozo`.
///
/// # Arguments
///
/// * `class_hash` - Class Hash of the model.
fn deploy_and_get_name_hash(salt: felt252, class_hash: starknet::ClassHash) -> SyscallResult<(starknet::ContractAddress, felt252)> {
    let (address, _) = starknet::deploy_syscall(
        class_hash,
        salt,
        array![].span(),
        false,
    )?;

    let name_hash = *starknet::call_contract_syscall(
        address,
        selector!("name_hash"),
        array![].span()
    )?[0];

    Result::Ok((address, name_hash))
}
