// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_verify_mac_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::verify_mac::VerifyMacInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.message {
        object.key("Message").string_unchecked(&::aws_smithy_types::base64::encode(var_1));
    }
    if let Some(var_2) = &input.key_id {
        object.key("KeyId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.mac_algorithm {
        object.key("MacAlgorithm").string(var_3.as_str());
    }
    if let Some(var_4) = &input.mac {
        object.key("Mac").string_unchecked(&::aws_smithy_types::base64::encode(var_4));
    }
    if let Some(var_5) = &input.grant_tokens {
        let mut array_6 = object.key("GrantTokens").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.dry_run {
        object.key("DryRun").boolean(*var_8);
    }
    Ok(())
}
