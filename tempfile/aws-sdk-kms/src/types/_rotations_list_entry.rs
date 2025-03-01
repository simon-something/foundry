// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about completed key material rotations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RotationsListEntry {
    /// <p>Unique identifier of the key.</p>
    pub key_id: ::std::option::Option<::std::string::String>,
    /// <p>Date and time that the key material rotation completed. Formatted as Unix time.</p>
    pub rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Identifies whether the key material rotation was a scheduled <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-enable-disable">automatic rotation</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-on-demand">on-demand rotation</a>.</p>
    pub rotation_type: ::std::option::Option<crate::types::RotationType>,
}
impl RotationsListEntry {
    /// <p>Unique identifier of the key.</p>
    pub fn key_id(&self) -> ::std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>Date and time that the key material rotation completed. Formatted as Unix time.</p>
    pub fn rotation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.rotation_date.as_ref()
    }
    /// <p>Identifies whether the key material rotation was a scheduled <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-enable-disable">automatic rotation</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-on-demand">on-demand rotation</a>.</p>
    pub fn rotation_type(&self) -> ::std::option::Option<&crate::types::RotationType> {
        self.rotation_type.as_ref()
    }
}
impl RotationsListEntry {
    /// Creates a new builder-style object to manufacture [`RotationsListEntry`](crate::types::RotationsListEntry).
    pub fn builder() -> crate::types::builders::RotationsListEntryBuilder {
        crate::types::builders::RotationsListEntryBuilder::default()
    }
}

/// A builder for [`RotationsListEntry`](crate::types::RotationsListEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RotationsListEntryBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
    pub(crate) rotation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) rotation_type: ::std::option::Option<crate::types::RotationType>,
}
impl RotationsListEntryBuilder {
    /// <p>Unique identifier of the key.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique identifier of the key.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>Unique identifier of the key.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_id
    }
    /// <p>Date and time that the key material rotation completed. Formatted as Unix time.</p>
    pub fn rotation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.rotation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date and time that the key material rotation completed. Formatted as Unix time.</p>
    pub fn set_rotation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.rotation_date = input;
        self
    }
    /// <p>Date and time that the key material rotation completed. Formatted as Unix time.</p>
    pub fn get_rotation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.rotation_date
    }
    /// <p>Identifies whether the key material rotation was a scheduled <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-enable-disable">automatic rotation</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-on-demand">on-demand rotation</a>.</p>
    pub fn rotation_type(mut self, input: crate::types::RotationType) -> Self {
        self.rotation_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Identifies whether the key material rotation was a scheduled <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-enable-disable">automatic rotation</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-on-demand">on-demand rotation</a>.</p>
    pub fn set_rotation_type(mut self, input: ::std::option::Option<crate::types::RotationType>) -> Self {
        self.rotation_type = input;
        self
    }
    /// <p>Identifies whether the key material rotation was a scheduled <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-enable-disable">automatic rotation</a> or an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html#rotating-keys-on-demand">on-demand rotation</a>.</p>
    pub fn get_rotation_type(&self) -> &::std::option::Option<crate::types::RotationType> {
        &self.rotation_type
    }
    /// Consumes the builder and constructs a [`RotationsListEntry`](crate::types::RotationsListEntry).
    pub fn build(self) -> crate::types::RotationsListEntry {
        crate::types::RotationsListEntry {
            key_id: self.key_id,
            rotation_date: self.rotation_date,
            rotation_type: self.rotation_type,
        }
    }
}
