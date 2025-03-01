// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListKeyRotationsOutput {
    /// <p>A list of completed key material rotations.</p>
    pub rotations: ::std::option::Option<::std::vec::Vec<crate::types::RotationsListEntry>>,
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    pub next_marker: ::std::option::Option<::std::string::String>,
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    pub truncated: bool,
    _request_id: Option<String>,
}
impl ListKeyRotationsOutput {
    /// <p>A list of completed key material rotations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.rotations.is_none()`.
    pub fn rotations(&self) -> &[crate::types::RotationsListEntry] {
        self.rotations.as_deref().unwrap_or_default()
    }
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn next_marker(&self) -> ::std::option::Option<&str> {
        self.next_marker.as_deref()
    }
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn truncated(&self) -> bool {
        self.truncated
    }
}
impl ::aws_types::request_id::RequestId for ListKeyRotationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListKeyRotationsOutput {
    /// Creates a new builder-style object to manufacture [`ListKeyRotationsOutput`](crate::operation::list_key_rotations::ListKeyRotationsOutput).
    pub fn builder() -> crate::operation::list_key_rotations::builders::ListKeyRotationsOutputBuilder {
        crate::operation::list_key_rotations::builders::ListKeyRotationsOutputBuilder::default()
    }
}

/// A builder for [`ListKeyRotationsOutput`](crate::operation::list_key_rotations::ListKeyRotationsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListKeyRotationsOutputBuilder {
    pub(crate) rotations: ::std::option::Option<::std::vec::Vec<crate::types::RotationsListEntry>>,
    pub(crate) next_marker: ::std::option::Option<::std::string::String>,
    pub(crate) truncated: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl ListKeyRotationsOutputBuilder {
    /// Appends an item to `rotations`.
    ///
    /// To override the contents of this collection use [`set_rotations`](Self::set_rotations).
    ///
    /// <p>A list of completed key material rotations.</p>
    pub fn rotations(mut self, input: crate::types::RotationsListEntry) -> Self {
        let mut v = self.rotations.unwrap_or_default();
        v.push(input);
        self.rotations = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of completed key material rotations.</p>
    pub fn set_rotations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RotationsListEntry>>) -> Self {
        self.rotations = input;
        self
    }
    /// <p>A list of completed key material rotations.</p>
    pub fn get_rotations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RotationsListEntry>> {
        &self.rotations
    }
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn next_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn set_next_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_marker = input;
        self
    }
    /// <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn get_next_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_marker
    }
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn truncated(mut self, input: bool) -> Self {
        self.truncated = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn set_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.truncated = input;
        self
    }
    /// <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    pub fn get_truncated(&self) -> &::std::option::Option<bool> {
        &self.truncated
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListKeyRotationsOutput`](crate::operation::list_key_rotations::ListKeyRotationsOutput).
    pub fn build(self) -> crate::operation::list_key_rotations::ListKeyRotationsOutput {
        crate::operation::list_key_rotations::ListKeyRotationsOutput {
            rotations: self.rotations,
            next_marker: self.next_marker,
            truncated: self.truncated.unwrap_or_default(),
            _request_id: self._request_id,
        }
    }
}
