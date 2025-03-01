// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_data_key_without_plaintext_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.key_id {
        object.key("KeyId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_context {
        #[allow(unused_mut)]
        let mut object_3 = object.key("EncryptionContext").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.key_spec {
        object.key("KeySpec").string(var_6.as_str());
    }
    if let Some(var_7) = &input.number_of_bytes {
        object.key("NumberOfBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.grant_tokens {
        let mut array_9 = object.key("GrantTokens").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.dry_run {
        object.key("DryRun").boolean(*var_11);
    }
    Ok(())
}
