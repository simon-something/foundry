// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::verify_mac::_verify_mac_output::VerifyMacOutputBuilder;

pub use crate::operation::verify_mac::_verify_mac_input::VerifyMacInputBuilder;

impl crate::operation::verify_mac::builders::VerifyMacInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::verify_mac::VerifyMacOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::verify_mac::VerifyMacError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.verify_mac();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `VerifyMac`.
///
/// <p>Verifies the hash-based message authentication code (HMAC) for a specified message, HMAC KMS key, and MAC algorithm. To verify the HMAC, <code>VerifyMac</code> computes an HMAC using the message, HMAC KMS key, and MAC algorithm that you specify, and compares the computed HMAC to the HMAC that you specify. If the HMACs are identical, the verification succeeds; otherwise, it fails. Verification indicates that the message hasn't changed since the HMAC was calculated, and the specified key was used to generate and verify the HMAC.</p>
/// <p>HMAC KMS keys and the HMAC algorithms that KMS uses conform to industry standards defined in <a href="https://datatracker.ietf.org/doc/html/rfc2104">RFC 2104</a>.</p>
/// <p>This operation is part of KMS support for HMAC KMS keys. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p><b>Cross-account use</b>: Yes. To perform this operation with a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p>
/// <p><b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:VerifyMac</a> (key policy)</p>
/// <p><b>Related operations</b>: <code>GenerateMac</code></p>
/// <p><b>Eventual consistency</b>: The KMS API follows an eventual consistency model. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-eventual-consistency.html">KMS eventual consistency</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct VerifyMacFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::verify_mac::builders::VerifyMacInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::verify_mac::VerifyMacOutput, crate::operation::verify_mac::VerifyMacError>
    for VerifyMacFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::verify_mac::VerifyMacOutput, crate::operation::verify_mac::VerifyMacError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl VerifyMacFluentBuilder {
    /// Creates a new `VerifyMacFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the VerifyMac as a reference.
    pub fn as_input(&self) -> &crate::operation::verify_mac::builders::VerifyMacInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::verify_mac::VerifyMacOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::verify_mac::VerifyMacError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::verify_mac::VerifyMac::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::verify_mac::VerifyMac::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::verify_mac::VerifyMacOutput,
        crate::operation::verify_mac::VerifyMacError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p><code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn message(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.message(input);
        self
    }
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p><code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_message(input);
        self
    }
    /// <p>The message that will be used in the verification. Enter the same message that was used to generate the HMAC.</p>
    /// <p><code>GenerateMac</code> and <code>VerifyMac</code> do not provide special handling for message digests. If you generated an HMAC for a hash digest of a message, you must verify the HMAC for the same hash digest.</p>
    pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_message()
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>The KMS key that will be used in the verification.</p>
    /// <p>Enter a key ID of the KMS key that was used to generate the HMAC. If you identify a different KMS key, the <code>VerifyMac</code> operation fails.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn mac_algorithm(mut self, input: crate::types::MacAlgorithmSpec) -> Self {
        self.inner = self.inner.mac_algorithm(input);
        self
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn set_mac_algorithm(mut self, input: ::std::option::Option<crate::types::MacAlgorithmSpec>) -> Self {
        self.inner = self.inner.set_mac_algorithm(input);
        self
    }
    /// <p>The MAC algorithm that will be used in the verification. Enter the same MAC algorithm that was used to compute the HMAC. This algorithm must be supported by the HMAC KMS key identified by the <code>KeyId</code> parameter.</p>
    pub fn get_mac_algorithm(&self) -> &::std::option::Option<crate::types::MacAlgorithmSpec> {
        self.inner.get_mac_algorithm()
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn mac(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.mac(input);
        self
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn set_mac(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_mac(input);
        self
    }
    /// <p>The HMAC to verify. Enter the HMAC that was generated by the <code>GenerateMac</code> operation when you specified the same message, HMAC KMS key, and MAC algorithm as the values specified in this request.</p>
    pub fn get_mac(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_mac()
    }
    ///
    /// Appends an item to `GrantTokens`.
    ///
    /// To override the contents of this collection use [`set_grant_tokens`](Self::set_grant_tokens).
    ///
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_tokens(input.into());
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_grant_tokens(input);
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_grant_tokens()
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-dryrun.html">Testing your KMS API calls</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-dryrun.html">Testing your KMS API calls</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-dryrun.html">Testing your KMS API calls</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
