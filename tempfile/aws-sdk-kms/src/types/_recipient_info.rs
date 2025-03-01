// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the party that receives the response from the API operation.</p>
/// <p>This data type is designed to support Amazon Web Services Nitro Enclaves, which lets you create an isolated compute environment in Amazon EC2. For information about the interaction between KMS and Amazon Web Services Nitro Enclaves, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/services-nitro-enclaves.html">How Amazon Web Services Nitro Enclaves uses KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecipientInfo {
    /// <p>The encryption algorithm that KMS should use with the public key for an Amazon Web Services Nitro Enclave to encrypt plaintext values for the response. The only valid value is <code>RSAES_OAEP_SHA_256</code>.</p>
    pub key_encryption_algorithm: ::std::option::Option<crate::types::KeyEncryptionMechanism>,
    /// <p>The attestation document for an Amazon Web Services Nitro Enclave. This document includes the enclave's public key.</p>
    pub attestation_document: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RecipientInfo {
    /// <p>The encryption algorithm that KMS should use with the public key for an Amazon Web Services Nitro Enclave to encrypt plaintext values for the response. The only valid value is <code>RSAES_OAEP_SHA_256</code>.</p>
    pub fn key_encryption_algorithm(&self) -> ::std::option::Option<&crate::types::KeyEncryptionMechanism> {
        self.key_encryption_algorithm.as_ref()
    }
    /// <p>The attestation document for an Amazon Web Services Nitro Enclave. This document includes the enclave's public key.</p>
    pub fn attestation_document(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.attestation_document.as_ref()
    }
}
impl RecipientInfo {
    /// Creates a new builder-style object to manufacture [`RecipientInfo`](crate::types::RecipientInfo).
    pub fn builder() -> crate::types::builders::RecipientInfoBuilder {
        crate::types::builders::RecipientInfoBuilder::default()
    }
}

/// A builder for [`RecipientInfo`](crate::types::RecipientInfo).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RecipientInfoBuilder {
    pub(crate) key_encryption_algorithm: ::std::option::Option<crate::types::KeyEncryptionMechanism>,
    pub(crate) attestation_document: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RecipientInfoBuilder {
    /// <p>The encryption algorithm that KMS should use with the public key for an Amazon Web Services Nitro Enclave to encrypt plaintext values for the response. The only valid value is <code>RSAES_OAEP_SHA_256</code>.</p>
    pub fn key_encryption_algorithm(mut self, input: crate::types::KeyEncryptionMechanism) -> Self {
        self.key_encryption_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption algorithm that KMS should use with the public key for an Amazon Web Services Nitro Enclave to encrypt plaintext values for the response. The only valid value is <code>RSAES_OAEP_SHA_256</code>.</p>
    pub fn set_key_encryption_algorithm(mut self, input: ::std::option::Option<crate::types::KeyEncryptionMechanism>) -> Self {
        self.key_encryption_algorithm = input;
        self
    }
    /// <p>The encryption algorithm that KMS should use with the public key for an Amazon Web Services Nitro Enclave to encrypt plaintext values for the response. The only valid value is <code>RSAES_OAEP_SHA_256</code>.</p>
    pub fn get_key_encryption_algorithm(&self) -> &::std::option::Option<crate::types::KeyEncryptionMechanism> {
        &self.key_encryption_algorithm
    }
    /// <p>The attestation document for an Amazon Web Services Nitro Enclave. This document includes the enclave's public key.</p>
    pub fn attestation_document(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.attestation_document = ::std::option::Option::Some(input);
        self
    }
    /// <p>The attestation document for an Amazon Web Services Nitro Enclave. This document includes the enclave's public key.</p>
    pub fn set_attestation_document(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.attestation_document = input;
        self
    }
    /// <p>The attestation document for an Amazon Web Services Nitro Enclave. This document includes the enclave's public key.</p>
    pub fn get_attestation_document(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.attestation_document
    }
    /// Consumes the builder and constructs a [`RecipientInfo`](crate::types::RecipientInfo).
    pub fn build(self) -> crate::types::RecipientInfo {
        crate::types::RecipientInfo {
            key_encryption_algorithm: self.key_encryption_algorithm,
            attestation_document: self.attestation_document,
        }
    }
}
