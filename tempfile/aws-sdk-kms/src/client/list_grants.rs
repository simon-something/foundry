// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGrants`](crate::operation::list_grants::builders::ListGrantsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_limit):<br>required: **false**<br><p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p> <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_marker):<br>required: **false**<br><p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p><br>
    ///   - [`key_id(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::key_id) / [`set_key_id(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_key_id):<br>required: **true**<br><p>Returns only grants for the specified KMS key. This parameter is required.</p> <p>Specify the key ID or key ARN of the KMS key. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN.</p> <p>For example:</p> <ul>  <li>   <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>  <li>   <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li> </ul> <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p><br>
    ///   - [`grant_id(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grant_id) / [`set_grant_id(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grant_id):<br>required: **false**<br><p>Returns only the grant with the specified grant ID. The grant ID uniquely identifies the grant.</p><br>
    ///   - [`grantee_principal(impl Into<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grantee_principal):<br>required: **false**<br><p>Returns only grants where the specified principal is the grantee principal for the grant.</p><br>
    /// - On success, responds with [`ListGrantsOutput`](crate::operation::list_grants::ListGrantsOutput) with field(s):
    ///   - [`grants(Option<Vec::<GrantListEntry>>)`](crate::operation::list_grants::ListGrantsOutput::grants): <p>A list of grants.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_grants::ListGrantsOutput::next_marker): <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    ///   - [`truncated(bool)`](crate::operation::list_grants::ListGrantsOutput::truncated): <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in this response to the <code>Marker</code> parameter in a subsequent request.</p>
    /// - On failure, responds with [`SdkError<ListGrantsError>`](crate::operation::list_grants::ListGrantsError)
    pub fn list_grants(&self) -> crate::operation::list_grants::builders::ListGrantsFluentBuilder {
        crate::operation::list_grants::builders::ListGrantsFluentBuilder::new(self.handle.clone())
    }
}
