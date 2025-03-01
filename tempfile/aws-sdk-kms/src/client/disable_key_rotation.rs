// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableKeyRotation`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies a symmetric encryption KMS key. You cannot enable or disable automatic rotation of <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html#asymmetric-cmks">asymmetric KMS keys</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC KMS keys</a>, KMS keys with <a href="https://docs.aws.amazon.com/kms/latest/developerguide/importing-keys.html">imported key material</a>, or KMS keys in a <a href="https://docs.aws.amazon.com/kms/latest/developerguide/custom-key-store-overview.html">custom key store</a>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
    /// - On success, responds with [`DisableKeyRotationOutput`](crate::operation::disable_key_rotation::DisableKeyRotationOutput)
    /// - On failure, responds with [`SdkError<DisableKeyRotationError>`](crate::operation::disable_key_rotation::DisableKeyRotationError)
    pub fn disable_key_rotation(&self) -> crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder {
        crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::new(self.handle.clone())
    }
}
