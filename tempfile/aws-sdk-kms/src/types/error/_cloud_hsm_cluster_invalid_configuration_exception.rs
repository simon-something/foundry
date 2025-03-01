// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected because the associated CloudHSM cluster did not meet the configuration requirements for an CloudHSM key store.</p>
/// <ul>
/// <li>
/// <p>The CloudHSM cluster must be configured with private subnets in at least two different Availability Zones in the Region.</p></li>
/// <li>
/// <p>The <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">security group for the cluster</a> (cloudhsm-cluster-<i><cluster-id></cluster-id></i>-sg) must include inbound rules and outbound rules that allow TCP traffic on ports 2223-2225. The <b>Source</b> in the inbound rules and the <b>Destination</b> in the outbound rules must match the security group ID. These rules are set by default when you create the CloudHSM cluster. Do not delete or change them. To get information about a particular security group, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSecurityGroups.html">DescribeSecurityGroups</a> operation.</p></li>
/// <li>
/// <p>The CloudHSM cluster must contain at least as many HSMs as the operation requires. To add HSMs, use the CloudHSM <a href="https://docs.aws.amazon.com/cloudhsm/latest/APIReference/API_CreateHsm.html">CreateHsm</a> operation.</p>
/// <p>For the <code>CreateCustomKeyStore</code>, <code>UpdateCustomKeyStore</code>, and <code>CreateKey</code> operations, the CloudHSM cluster must have at least two active HSMs, each in a different Availability Zone. For the <code>ConnectCustomKeyStore</code> operation, the CloudHSM must contain at least one active HSM.</p></li>
/// </ul>
/// <p>For information about the requirements for an CloudHSM cluster that is associated with an CloudHSM key store, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keystore.html#before-keystore">Assemble the Prerequisites</a> in the <i>Key Management Service Developer Guide</i>. For information about creating a private subnet for an CloudHSM cluster, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/create-subnets.html">Create a Private Subnet</a> in the <i>CloudHSM User Guide</i>. For information about cluster security groups, see <a href="https://docs.aws.amazon.com/cloudhsm/latest/userguide/configure-sg.html">Configure a Default Security Group</a> in the <i> <i>CloudHSM User Guide</i> </i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudHsmClusterInvalidConfigurationException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl CloudHsmClusterInvalidConfigurationException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for CloudHsmClusterInvalidConfigurationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "CloudHsmClusterInvalidConfigurationException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for CloudHsmClusterInvalidConfigurationException {}
impl ::aws_types::request_id::RequestId for crate::types::error::CloudHsmClusterInvalidConfigurationException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for CloudHsmClusterInvalidConfigurationException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl CloudHsmClusterInvalidConfigurationException {
    /// Creates a new builder-style object to manufacture [`CloudHsmClusterInvalidConfigurationException`](crate::types::error::CloudHsmClusterInvalidConfigurationException).
    pub fn builder() -> crate::types::error::builders::CloudHsmClusterInvalidConfigurationExceptionBuilder {
        crate::types::error::builders::CloudHsmClusterInvalidConfigurationExceptionBuilder::default()
    }
}

/// A builder for [`CloudHsmClusterInvalidConfigurationException`](crate::types::error::CloudHsmClusterInvalidConfigurationException).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CloudHsmClusterInvalidConfigurationExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl CloudHsmClusterInvalidConfigurationExceptionBuilder {
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
    /// Consumes the builder and constructs a [`CloudHsmClusterInvalidConfigurationException`](crate::types::error::CloudHsmClusterInvalidConfigurationException).
    pub fn build(self) -> crate::types::error::CloudHsmClusterInvalidConfigurationException {
        crate::types::error::CloudHsmClusterInvalidConfigurationException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
