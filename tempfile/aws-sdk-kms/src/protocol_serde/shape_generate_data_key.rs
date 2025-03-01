// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_data_key_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::generate_data_key::GenerateDataKeyOutput, crate::operation::generate_data_key::GenerateDataKeyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DependencyTimeoutException" => crate::operation::generate_data_key::GenerateDataKeyError::DependencyTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DependencyTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DisabledException" => crate::operation::generate_data_key::GenerateDataKeyError::DisabledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DisabledExceptionBuilder::default();
                output = crate::protocol_serde::shape_disabled_exception::de_disabled_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DryRunOperationException" => crate::operation::generate_data_key::GenerateDataKeyError::DryRunOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DryRunOperationExceptionBuilder::default();
                output = crate::protocol_serde::shape_dry_run_operation_exception::de_dry_run_operation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidGrantTokenException" => crate::operation::generate_data_key::GenerateDataKeyError::InvalidGrantTokenException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidGrantTokenExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_grant_token_exception::de_invalid_grant_token_exception_json_err(_response_body, output)
                        .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidKeyUsageException" => crate::operation::generate_data_key::GenerateDataKeyError::InvalidKeyUsageException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidKeyUsageExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_key_usage_exception::de_invalid_key_usage_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KeyUnavailableException" => crate::operation::generate_data_key::GenerateDataKeyError::KeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KeyUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_key_unavailable_exception::de_key_unavailable_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KMSInternalException" => crate::operation::generate_data_key::GenerateDataKeyError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KMSInvalidStateException" => crate::operation::generate_data_key::GenerateDataKeyError::KmsInvalidStateException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::generate_data_key::GenerateDataKeyError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::generate_data_key::GenerateDataKeyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_data_key_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::generate_data_key::GenerateDataKeyOutput, crate::operation::generate_data_key::GenerateDataKeyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::generate_data_key::builders::GenerateDataKeyOutputBuilder::default();
        output = crate::protocol_serde::shape_generate_data_key::de_generate_data_key(_response_body, output)
            .map_err(crate::operation::generate_data_key::GenerateDataKeyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_generate_data_key_input(
    input: &crate::operation::generate_data_key::GenerateDataKeyInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_generate_data_key_input::ser_generate_data_key_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_generate_data_key(
    value: &[u8],
    mut builder: crate::operation::generate_data_key::builders::GenerateDataKeyOutputBuilder,
) -> Result<crate::operation::generate_data_key::builders::GenerateDataKeyOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CiphertextBlob" => {
                    builder = builder.set_ciphertext_blob(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
                }
                "Plaintext" => {
                    builder = builder.set_plaintext(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
                }
                "KeyId" => {
                    builder = builder.set_key_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "CiphertextForRecipient" => {
                    builder = builder.set_ciphertext_for_recipient(::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?);
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
