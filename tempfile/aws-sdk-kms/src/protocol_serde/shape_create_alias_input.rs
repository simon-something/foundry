// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_alias_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_alias::CreateAliasInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.alias_name {
        object.key("AliasName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_key_id {
        object.key("TargetKeyId").string(var_2.as_str());
    }
    Ok(())
}
