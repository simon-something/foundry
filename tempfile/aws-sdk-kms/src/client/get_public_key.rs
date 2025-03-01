// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPublicKey`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_key_id):<br>required: **true**<br><p>Identifies the asymmetric KMS key that includes the public key.</p> <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Alias name: <code>alias/ExampleAlias</code></p></li>  <li>   <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p><br>
    ///   - [`grant_tokens(impl Into<String>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<Vec::<String>>)`](crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::set_grant_tokens):<br>required: **false**<br><p>A list of grant tokens.</p> <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grant-manage.html#using-grant-token">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p><br>
    /// - On success, responds with [`GetPublicKeyOutput`](crate::operation::get_public_key::GetPublicKeyOutput) with field(s):
    ///   - [`key_id(Option<String>)`](crate::operation::get_public_key::GetPublicKeyOutput::key_id): <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-ARN">key ARN</a>) of the asymmetric KMS key from which the public key was downloaded.</p>
    ///   - [`public_key(Option<Blob>)`](crate::operation::get_public_key::GetPublicKeyOutput::public_key): <p>The exported public key.</p> <p>The value is a DER-encoded X.509 public key, also known as <code>SubjectPublicKeyInfo</code> (SPKI), as defined in <a href="https://tools.ietf.org/html/rfc5280">RFC 5280</a>. When you use the HTTP API or the Amazon Web Services CLI, the value is Base64-encoded. Otherwise, it is not Base64-encoded.</p> <p></p>
    ///   - [`customer_master_key_spec(Option<CustomerMasterKeySpec>)`](crate::operation::get_public_key::GetPublicKeyOutput::customer_master_key_spec): <p>Instead, use the <code>KeySpec</code> field in the <code>GetPublicKey</code> response.</p> <p>The <code>KeySpec</code> and <code>CustomerMasterKeySpec</code> fields have the same value. We recommend that you use the <code>KeySpec</code> field in your code. However, to avoid breaking changes, KMS supports both fields.</p>
    ///   - [`key_spec(Option<KeySpec>)`](crate::operation::get_public_key::GetPublicKeyOutput::key_spec): <p>The type of the of the public key that was downloaded.</p>
    ///   - [`key_usage(Option<KeyUsageType>)`](crate::operation::get_public_key::GetPublicKeyOutput::key_usage): <p>The permitted use of the public key. Valid values for asymmetric key pairs are <code>ENCRYPT_DECRYPT</code>, <code>SIGN_VERIFY</code>, and <code>KEY_AGREEMENT</code>.</p> <p>This information is critical. For example, if a public key with <code>SIGN_VERIFY</code> key usage encrypts data outside of KMS, the ciphertext cannot be decrypted.</p>
    ///   - [`encryption_algorithms(Option<Vec::<EncryptionAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyOutput::encryption_algorithms): <p>The encryption algorithms that KMS supports for this key.</p> <p>This information is critical. If a public key encrypts data outside of KMS by using an unsupported encryption algorithm, the ciphertext cannot be decrypted.</p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>ENCRYPT_DECRYPT</code>.</p>
    ///   - [`signing_algorithms(Option<Vec::<SigningAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyOutput::signing_algorithms): <p>The signing algorithms that KMS supports for this key.</p> <p>This field appears in the response only when the <code>KeyUsage</code> of the public key is <code>SIGN_VERIFY</code>.</p>
    ///   - [`key_agreement_algorithms(Option<Vec::<KeyAgreementAlgorithmSpec>>)`](crate::operation::get_public_key::GetPublicKeyOutput::key_agreement_algorithms): <p>The key agreement algorithm used to derive a shared secret. This field is present only when the KMS key has a <code>KeyUsage</code> value of <code>KEY_AGREEMENT</code>.</p>
    /// - On failure, responds with [`SdkError<GetPublicKeyError>`](crate::operation::get_public_key::GetPublicKeyError)
    pub fn get_public_key(&self) -> crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder {
        crate::operation::get_public_key::builders::GetPublicKeyFluentBuilder::new(self.handle.clone())
    }
}
