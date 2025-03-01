// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected because the specified CloudHSM cluster is already associated with an CloudHSM key store in the account, or it shares a backup history with an CloudHSM key store in the account. Each CloudHSM key store in the account must be associated with a different CloudHSM cluster.</p>
/// <p>CloudHSM clusters that share a backup history have the same cluster certificate. To view the cluster certificate of an CloudHSM cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudHsmClusterInUseException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl CloudHsmClusterInUseException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for CloudHsmClusterInUseException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "CloudHsmClusterInUseException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for CloudHsmClusterInUseException {}
impl ::aws_types::request_id::RequestId for crate::types::error::CloudHsmClusterInUseException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CloudHsmClusterInUseException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl CloudHsmClusterInUseException {
    /// Creates a new builder-style object to manufacture [`CloudHsmClusterInUseException`](crate::types::error::CloudHsmClusterInUseException).
    pub fn builder() -> crate::types::error::builders::CloudHsmClusterInUseExceptionBuilder {
        crate::types::error::builders::CloudHsmClusterInUseExceptionBuilder::default()
    }
}

/// A builder for [`CloudHsmClusterInUseException`](crate::types::error::CloudHsmClusterInUseException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CloudHsmClusterInUseExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl CloudHsmClusterInUseExceptionBuilder {
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
    /// Consumes the builder and constructs a [`CloudHsmClusterInUseException`](crate::types::error::CloudHsmClusterInUseException).
    pub fn build(self) -> crate::types::error::CloudHsmClusterInUseException {
        crate::types::error::CloudHsmClusterInUseException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
