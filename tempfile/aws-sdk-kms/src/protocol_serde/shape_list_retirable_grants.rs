// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_retirable_grants_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_retirable_grants::ListRetirableGrantsOutput,
    crate::operation::list_retirable_grants::ListRetirableGrantsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DependencyTimeoutException" => crate::operation::list_retirable_grants::ListRetirableGrantsError::DependencyTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DependencyTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_dependency_timeout_exception::de_dependency_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArnException" => crate::operation::list_retirable_grants::ListRetirableGrantsError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArnExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidMarkerException" => crate::operation::list_retirable_grants::ListRetirableGrantsError::InvalidMarkerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidMarkerExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_marker_exception::de_invalid_marker_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KMSInternalException" => crate::operation::list_retirable_grants::ListRetirableGrantsError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::list_retirable_grants::ListRetirableGrantsError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_retirable_grants::ListRetirableGrantsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_retirable_grants_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_retirable_grants::ListRetirableGrantsOutput,
    crate::operation::list_retirable_grants::ListRetirableGrantsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_retirable_grants::builders::ListRetirableGrantsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_retirable_grants::de_list_retirable_grants(_response_body, output)
            .map_err(crate::operation::list_retirable_grants::ListRetirableGrantsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_retirable_grants_input(
    input: &crate::operation::list_retirable_grants::ListRetirableGrantsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_retirable_grants_input::ser_list_retirable_grants_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_list_retirable_grants(
    value: &[u8],
    mut builder: crate::operation::list_retirable_grants::builders::ListRetirableGrantsOutputBuilder,
) -> Result<
    crate::operation::list_retirable_grants::builders::ListRetirableGrantsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Grants" => {
                    builder = builder.set_grants(crate::protocol_serde::shape_grant_list::de_grant_list(tokens)?);
                }
                "NextMarker" => {
                    builder = builder.set_next_marker(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Truncated" => {
                    builder = builder.set_truncated(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
