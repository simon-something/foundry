// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_custom_key_store_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_custom_key_store::DeleteCustomKeyStoreInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.custom_key_store_id {
        object.key("CustomKeyStoreId").string(var_1.as_str());
    }
    Ok(())
}
