// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateAlias`](crate::operation::create_alias::builders::CreateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias_name(impl Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_alias_name):<br>required: **true**<br><p>Specifies the alias name. This value must begin with <code>alias/</code> followed by a name, such as <code>alias/ExampleAlias</code>.</p><important>  <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p> </important> <p>The <code>AliasName</code> value must be string of 1-256 characters. It can contain only alphanumeric characters, forward slashes (/), underscores (_), and dashes (-). The alias name cannot begin with <code>alias/aws/</code>. The <code>alias/aws/</code> prefix is reserved for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk">Amazon Web Services managed keys</a>.</p><br>
    ///   - [`target_key_id(impl Into<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_target_key_id):<br>required: **true**<br><p>Associates the alias with the specified <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#customer-cmk">customer managed key</a>. The KMS key must be in the same Amazon Web Services Region.</p> <p>A valid key ID is required. If you supply a null or empty string value, this operation returns an error.</p> <p>For help finding the key ID and ARN, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/viewing-keys.html#find-cmk-id-arn">Finding the Key ID and ARN</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p> <p>Specify the key ID or key ARN of the KMS key.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
    /// - On success, responds with [`CreateAliasOutput`](crate::operation::create_alias::CreateAliasOutput)
    /// - On failure, responds with [`SdkError<CreateAliasError>`](crate::operation::create_alias::CreateAliasError)
    pub fn create_alias(&self) -> crate::operation::create_alias::builders::CreateAliasFluentBuilder {
        crate::operation::create_alias::builders::CreateAliasFluentBuilder::new(self.handle.clone())
    }
}
