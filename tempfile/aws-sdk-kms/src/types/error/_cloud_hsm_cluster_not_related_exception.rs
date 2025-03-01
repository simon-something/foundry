// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected because the specified CloudHSM cluster has a different cluster certificate than the original cluster. You cannot use the operation to specify an unrelated cluster for an CloudHSM key store.</p>
/// <p>Specify an CloudHSM cluster that shares a backup history with the original cluster. This includes clusters that were created from a backup of the current cluster, and clusters that were created from the same backup that produced the current cluster.</p>
/// <p>CloudHSM clusters that share a backup history have the same cluster certificate. To view the cluster certificate of an CloudHSM cluster, use the <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_DescribeClusters.html">DescribeClusters</a> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudHsmClusterNotRelatedException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl CloudHsmClusterNotRelatedException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for CloudHsmClusterNotRelatedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "CloudHsmClusterNotRelatedException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for CloudHsmClusterNotRelatedException {}
impl ::aws_types::request_id::RequestId for crate::types::error::CloudHsmClusterNotRelatedException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CloudHsmClusterNotRelatedException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl CloudHsmClusterNotRelatedException {
    /// Creates a new builder-style object to manufacture [`CloudHsmClusterNotRelatedException`](crate::types::error::CloudHsmClusterNotRelatedException).
    pub fn builder() -> crate::types::error::builders::CloudHsmClusterNotRelatedExceptionBuilder {
        crate::types::error::builders::CloudHsmClusterNotRelatedExceptionBuilder::default()
    }
}

/// A builder for [`CloudHsmClusterNotRelatedException`](crate::types::error::CloudHsmClusterNotRelatedException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CloudHsmClusterNotRelatedExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl CloudHsmClusterNotRelatedExceptionBuilder {
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
    /// Consumes the builder and constructs a [`CloudHsmClusterNotRelatedException`](crate::types::error::CloudHsmClusterNotRelatedException).
    pub fn build(self) -> crate::types::error::CloudHsmClusterNotRelatedException {
        crate::types::error::CloudHsmClusterNotRelatedException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
