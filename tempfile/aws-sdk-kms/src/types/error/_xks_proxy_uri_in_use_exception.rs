// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected because the concatenation of the <code>XksProxyUriEndpoint</code> and <code>XksProxyUriPath</code> is already associated with another external key store in this Amazon Web Services Region. Each external key store in a Region must use a unique external key store proxy API address.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct XksProxyUriInUseException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl XksProxyUriInUseException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for XksProxyUriInUseException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "XksProxyUriInUseException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for XksProxyUriInUseException {}
impl ::aws_types::request_id::RequestId for crate::types::error::XksProxyUriInUseException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for XksProxyUriInUseException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl XksProxyUriInUseException {
    /// Creates a new builder-style object to manufacture [`XksProxyUriInUseException`](crate::types::error::XksProxyUriInUseException).
    pub fn builder() -> crate::types::error::builders::XksProxyUriInUseExceptionBuilder {
        crate::types::error::builders::XksProxyUriInUseExceptionBuilder::default()
    }
}

/// A builder for [`XksProxyUriInUseException`](crate::types::error::XksProxyUriInUseException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct XksProxyUriInUseExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl XksProxyUriInUseExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`XksProxyUriInUseException`](crate::types::error::XksProxyUriInUseException).
    pub fn build(self) -> crate::types::error::XksProxyUriInUseException {
        crate::types::error::XksProxyUriInUseException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
