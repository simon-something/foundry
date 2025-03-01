// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetParametersForImportInput {
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub key_id: ::std::option::Option<::std::string::String>,
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub wrapping_algorithm: ::std::option::Option<crate::types::AlgorithmSpec>,
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub wrapping_key_spec: ::std::option::Option<crate::types::WrappingKeySpec>,
}
impl GetParametersForImportInput {
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(&self) -> ::std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn wrapping_algorithm(&self) -> ::std::option::Option<&crate::types::AlgorithmSpec> {
        self.wrapping_algorithm.as_ref()
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn wrapping_key_spec(&self) -> ::std::option::Option<&crate::types::WrappingKeySpec> {
        self.wrapping_key_spec.as_ref()
    }
}
impl GetParametersForImportInput {
    /// Creates a new builder-style object to manufacture [`GetParametersForImportInput`](crate::operation::get_parameters_for_import::GetParametersForImportInput).
    pub fn builder() -> crate::operation::get_parameters_for_import::builders::GetParametersForImportInputBuilder {
        crate::operation::get_parameters_for_import::builders::GetParametersForImportInputBuilder::default()
    }
}

/// A builder for [`GetParametersForImportInput`](crate::operation::get_parameters_for_import::GetParametersForImportInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetParametersForImportInputBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
    pub(crate) wrapping_algorithm: ::std::option::Option<crate::types::AlgorithmSpec>,
    pub(crate) wrapping_key_spec: ::std::option::Option<crate::types::WrappingKeySpec>,
}
impl GetParametersForImportInputBuilder {
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    /// This field is required.
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>The identifier of the KMS key that will be associated with the imported key material. The <code>Origin</code> of the KMS key must be <code>EXTERNAL</code>.</p>
    /// <p>All KMS key types are supported, including multi-Region keys. However, you cannot import key material into a KMS key in a custom key store.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_id
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    /// This field is required.
    pub fn wrapping_algorithm(mut self, input: crate::types::AlgorithmSpec) -> Self {
        self.wrapping_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn set_wrapping_algorithm(mut self, input: ::std::option::Option<crate::types::AlgorithmSpec>) -> Self {
        self.wrapping_algorithm = input;
        self
    }
    /// <p>The algorithm you will use with the RSA public key (<code>PublicKey</code>) in the response to protect your key material during import. For more information, see <a href="kms/latest/developerguide/importing-keys-get-public-key-and-token.html#select-wrapping-algorithm">Select a wrapping algorithm</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// <p>For RSA_AES wrapping algorithms, you encrypt your key material with an AES key that you generate, then encrypt your AES key with the RSA public key from KMS. For RSAES wrapping algorithms, you encrypt your key material directly with the RSA public key from KMS.</p>
    /// <p>The wrapping algorithms that you can use depend on the type of key material that you are importing. To import an RSA private key, you must use an RSA_AES wrapping algorithm.</p>
    /// <ul>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_256</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSA_AES_KEY_WRAP_SHA_1</b> — Supported for wrapping RSA and ECC key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_256</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_256 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_OAEP_SHA_1</b> — Supported for all types of key material, except RSA key material (private key).</p>
    /// <p>You cannot use the RSAES_OAEP_SHA_1 wrapping algorithm with the RSA_2048 wrapping key spec to wrap ECC_NIST_P521 key material.</p></li>
    /// <li>
    /// <p><b>RSAES_PKCS1_V1_5</b> (Deprecated) — As of October 10, 2023, KMS does not support the RSAES_PKCS1_V1_5 wrapping algorithm.</p></li>
    /// </ul>
    pub fn get_wrapping_algorithm(&self) -> &::std::option::Option<crate::types::AlgorithmSpec> {
        &self.wrapping_algorithm
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    /// This field is required.
    pub fn wrapping_key_spec(mut self, input: crate::types::WrappingKeySpec) -> Self {
        self.wrapping_key_spec = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn set_wrapping_key_spec(mut self, input: ::std::option::Option<crate::types::WrappingKeySpec>) -> Self {
        self.wrapping_key_spec = input;
        self
    }
    /// <p>The type of RSA public key to return in the response. You will use this wrapping key with the specified wrapping algorithm to protect your key material during import.</p>
    /// <p>Use the longest RSA wrapping key that is practical.</p>
    /// <p>You cannot use an RSA_2048 public key to directly wrap an ECC_NIST_P521 private key. Instead, use an RSA_AES wrapping algorithm or choose a longer RSA public key.</p>
    pub fn get_wrapping_key_spec(&self) -> &::std::option::Option<crate::types::WrappingKeySpec> {
        &self.wrapping_key_spec
    }
    /// Consumes the builder and constructs a [`GetParametersForImportInput`](crate::operation::get_parameters_for_import::GetParametersForImportInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_parameters_for_import::GetParametersForImportInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_parameters_for_import::GetParametersForImportInput {
            key_id: self.key_id,
            wrapping_algorithm: self.wrapping_algorithm,
            wrapping_key_spec: self.wrapping_key_spec,
        })
    }
}
