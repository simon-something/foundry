// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about a grant.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GrantListEntry {
    /// <p>The unique identifier for the KMS key to which the grant applies.</p>
    pub key_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the grant.</p>
    pub grant_id: ::std::option::Option<::std::string::String>,
    /// <p>The friendly name that identifies the grant. If a name was provided in the <code>CreateGrant</code> request, that name is returned. Otherwise this value is null.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the grant was created.</p>
    pub creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The identity that gets the permissions in the grant.</p>
    /// <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an Amazon Web Services service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    pub grantee_principal: ::std::option::Option<::std::string::String>,
    /// <p>The principal that can retire the grant.</p>
    pub retiring_principal: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services account under which the grant was issued.</p>
    pub issuing_account: ::std::option::Option<::std::string::String>,
    /// <p>The list of operations permitted by the grant.</p>
    pub operations: ::std::option::Option<::std::vec::Vec<crate::types::GrantOperation>>,
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    pub constraints: ::std::option::Option<crate::types::GrantConstraints>,
}
impl GrantListEntry {
    /// <p>The unique identifier for the KMS key to which the grant applies.</p>
    pub fn key_id(&self) -> ::std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>The unique identifier for the grant.</p>
    pub fn grant_id(&self) -> ::std::option::Option<&str> {
        self.grant_id.as_deref()
    }
    /// <p>The friendly name that identifies the grant. If a name was provided in the <code>CreateGrant</code> request, that name is returned. Otherwise this value is null.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The date and time when the grant was created.</p>
    pub fn creation_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_date.as_ref()
    }
    /// <p>The identity that gets the permissions in the grant.</p>
    /// <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an Amazon Web Services service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    pub fn grantee_principal(&self) -> ::std::option::Option<&str> {
        self.grantee_principal.as_deref()
    }
    /// <p>The principal that can retire the grant.</p>
    pub fn retiring_principal(&self) -> ::std::option::Option<&str> {
        self.retiring_principal.as_deref()
    }
    /// <p>The Amazon Web Services account under which the grant was issued.</p>
    pub fn issuing_account(&self) -> ::std::option::Option<&str> {
        self.issuing_account.as_deref()
    }
    /// <p>The list of operations permitted by the grant.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.operations.is_none()`.
    pub fn operations(&self) -> &[crate::types::GrantOperation] {
        self.operations.as_deref().unwrap_or_default()
    }
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    pub fn constraints(&self) -> ::std::option::Option<&crate::types::GrantConstraints> {
        self.constraints.as_ref()
    }
}
impl GrantListEntry {
    /// Creates a new builder-style object to manufacture [`GrantListEntry`](crate::types::GrantListEntry).
    pub fn builder() -> crate::types::builders::GrantListEntryBuilder {
        crate::types::builders::GrantListEntryBuilder::default()
    }
}

/// A builder for [`GrantListEntry`](crate::types::GrantListEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GrantListEntryBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
    pub(crate) grant_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) grantee_principal: ::std::option::Option<::std::string::String>,
    pub(crate) retiring_principal: ::std::option::Option<::std::string::String>,
    pub(crate) issuing_account: ::std::option::Option<::std::string::String>,
    pub(crate) operations: ::std::option::Option<::std::vec::Vec<crate::types::GrantOperation>>,
    pub(crate) constraints: ::std::option::Option<crate::types::GrantConstraints>,
}
impl GrantListEntryBuilder {
    /// <p>The unique identifier for the KMS key to which the grant applies.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the KMS key to which the grant applies.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>The unique identifier for the KMS key to which the grant applies.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_id
    }
    /// <p>The unique identifier for the grant.</p>
    pub fn grant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grant_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the grant.</p>
    pub fn set_grant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grant_id = input;
        self
    }
    /// <p>The unique identifier for the grant.</p>
    pub fn get_grant_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.grant_id
    }
    /// <p>The friendly name that identifies the grant. If a name was provided in the <code>CreateGrant</code> request, that name is returned. Otherwise this value is null.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The friendly name that identifies the grant. If a name was provided in the <code>CreateGrant</code> request, that name is returned. Otherwise this value is null.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The friendly name that identifies the grant. If a name was provided in the <code>CreateGrant</code> request, that name is returned. Otherwise this value is null.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The date and time when the grant was created.</p>
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the grant was created.</p>
    pub fn set_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>The date and time when the grant was created.</p>
    pub fn get_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_date
    }
    /// <p>The identity that gets the permissions in the grant.</p>
    /// <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an Amazon Web Services service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    pub fn grantee_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.grantee_principal = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identity that gets the permissions in the grant.</p>
    /// <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an Amazon Web Services service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    pub fn set_grantee_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.grantee_principal = input;
        self
    }
    /// <p>The identity that gets the permissions in the grant.</p>
    /// <p>The <code>GranteePrincipal</code> field in the <code>ListGrants</code> response usually contains the user or role designated as the grantee principal in the grant. However, when the grantee principal in the grant is an Amazon Web Services service, the <code>GranteePrincipal</code> field contains the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html#principal-services">service principal</a>, which might represent several different grantee principals.</p>
    pub fn get_grantee_principal(&self) -> &::std::option::Option<::std::string::String> {
        &self.grantee_principal
    }
    /// <p>The principal that can retire the grant.</p>
    pub fn retiring_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.retiring_principal = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The principal that can retire the grant.</p>
    pub fn set_retiring_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.retiring_principal = input;
        self
    }
    /// <p>The principal that can retire the grant.</p>
    pub fn get_retiring_principal(&self) -> &::std::option::Option<::std::string::String> {
        &self.retiring_principal
    }
    /// <p>The Amazon Web Services account under which the grant was issued.</p>
    pub fn issuing_account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.issuing_account = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account under which the grant was issued.</p>
    pub fn set_issuing_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.issuing_account = input;
        self
    }
    /// <p>The Amazon Web Services account under which the grant was issued.</p>
    pub fn get_issuing_account(&self) -> &::std::option::Option<::std::string::String> {
        &self.issuing_account
    }
    /// Appends an item to `operations`.
    ///
    /// To override the contents of this collection use [`set_operations`](Self::set_operations).
    ///
    /// <p>The list of operations permitted by the grant.</p>
    pub fn operations(mut self, input: crate::types::GrantOperation) -> Self {
        let mut v = self.operations.unwrap_or_default();
        v.push(input);
        self.operations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of operations permitted by the grant.</p>
    pub fn set_operations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GrantOperation>>) -> Self {
        self.operations = input;
        self
    }
    /// <p>The list of operations permitted by the grant.</p>
    pub fn get_operations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GrantOperation>> {
        &self.operations
    }
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    pub fn constraints(mut self, input: crate::types::GrantConstraints) -> Self {
        self.constraints = ::std::option::Option::Some(input);
        self
    }
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    pub fn set_constraints(mut self, input: ::std::option::Option<crate::types::GrantConstraints>) -> Self {
        self.constraints = input;
        self
    }
    /// <p>A list of key-value pairs that must be present in the encryption context of certain subsequent operations that the grant allows.</p>
    pub fn get_constraints(&self) -> &::std::option::Option<crate::types::GrantConstraints> {
        &self.constraints
    }
    /// Consumes the builder and constructs a [`GrantListEntry`](crate::types::GrantListEntry).
    pub fn build(self) -> crate::types::GrantListEntry {
        crate::types::GrantListEntry {
            key_id: self.key_id,
            grant_id: self.grant_id,
            name: self.name,
            creation_date: self.creation_date,
            grantee_principal: self.grantee_principal,
            retiring_principal: self.retiring_principal,
            issuing_account: self.issuing_account,
            operations: self.operations,
            constraints: self.constraints,
        }
    }
}
