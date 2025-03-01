// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_key_rotation_status::_get_key_rotation_status_output::GetKeyRotationStatusOutputBuilder;

pub use crate::operation::get_key_rotation_status::_get_key_rotation_status_input::GetKeyRotationStatusInputBuilder;

impl crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_key_rotation_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetKeyRotationStatus`.
///
/// <p>Provides detailed information about the rotation status for a KMS key, including whether <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic rotation of the key material</a> is enabled for the specified KMS key, the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotation-period">rotation period</a>, and the next scheduled rotation date.</p>
/// <p>Automatic key rotation is supported only on <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#symmetric-cmks">symmetric encryption KMS keys</a>. You cannot enable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>. To enable or disable automatic rotation of a set of related <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-manage.html#multi-region-rotate">multi-Region keys</a>, set the property on the primary key..</p>
/// <p>You can enable (<code>EnableKeyRotation</code>) and disable automatic rotation (<code>DisableKeyRotation</code>) of the key material in customer managed KMS keys. Key material rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed KMS keys</a> is not configurable. KMS always rotates the key material in Amazon Web Services managed KMS keys every year. The key rotation status for Amazon Web Services managed KMS keys is always <code>true</code>.</p>
/// <p>You can perform on-demand (<code>RotateKeyOnDemand</code>) rotation of the key material in customer managed KMS keys, regardless of whether or not automatic key rotation is enabled. You can use GetKeyRotationStatus to identify the date and time that an in progress on-demand rotation was initiated. You can use <code>ListKeyRotations</code> to view the details of completed rotations.</p><note>
/// <p>In May 2022, KMS changed the rotation schedule for Amazon Web Services managed keys from every three years to every year. For details, see <code>EnableKeyRotation</code>.</p>
/// </note>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <ul>
/// <li>
/// <p>Disabled: The key rotation status does not change when you disable a KMS key. However, while the KMS key is disabled, KMS does not rotate the key material. When you re-enable the KMS key, rotation resumes. If the key material in the re-enabled KMS key hasn't been rotated in one year, KMS rotates it immediately, and every year thereafter. If it's been less than a year since the key material in the re-enabled KMS key was rotated, the KMS key resumes its prior rotation schedule.</p></li>
/// <li>
/// <p>Pending deletion: While a KMS key is pending deletion, its key rotation status is <code>false</code> and KMS does not rotate the key material. If you cancel the deletion, the original key rotation status returns to <code>true</code>.</p></li>
/// </ul>
/// <p><b>Cross-account use</b>: Yes. To perform this operation on a KMS key in a different Amazon Web Services account, specify the key ARN in the value of the <code>KeyId</code> parameter.</p>
/// <p><b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:GetKeyRotationStatus</a> (key policy)</p>
/// <p><b>Related operations:</b></p>
/// <ul>
/// <li>
/// <p><code>DisableKeyRotation</code></p></li>
/// <li>
/// <p><code>EnableKeyRotation</code></p></li>
/// <li>
/// <p><code>ListKeyRotations</code></p></li>
/// <li>
/// <p><code>RotateKeyOnDemand</code></p></li>
/// </ul>
/// <p><b>Eventual consistency</b>: The KMS API follows an eventual consistency model. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/programming-eventual-consistency.html">KMS eventual consistency</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetKeyRotationStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput,
        crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
    > for GetKeyRotationStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput,
            crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetKeyRotationStatusFluentBuilder {
    /// Creates a new `GetKeyRotationStatusFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetKeyRotationStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusInputBuilder {
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
        crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_key_rotation_status::GetKeyRotationStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_key_rotation_status::GetKeyRotationStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_key_rotation_status::GetKeyRotationStatusOutput,
        crate::operation::get_key_rotation_status::GetKeyRotationStatusError,
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
    /// <p>Gets the rotation status for the specified KMS key.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>Gets the rotation status for the specified KMS key.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>Gets the rotation status for the specified KMS key.</p>
    /// <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
}
