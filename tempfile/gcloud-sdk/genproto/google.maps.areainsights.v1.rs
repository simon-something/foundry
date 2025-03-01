// This file is @generated by prost-build.
/// Request for the ComputeInsights RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeInsightsRequest {
    /// Required. Insights to compute. Currently only INSIGHT_COUNT and
    /// INSIGHT_PLACES are supported.
    #[prost(enumeration = "Insight", repeated, packed = "false", tag = "4")]
    pub insights: ::prost::alloc::vec::Vec<i32>,
    /// Required. Insight filter.
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<Filter>,
}
/// Response for the ComputeInsights RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeInsightsResponse {
    /// Result for Insights.INSIGHT_COUNT.
    #[prost(int64, optional, tag = "1")]
    pub count: ::core::option::Option<i64>,
    /// Result for Insights.INSIGHT_PLACES.
    #[prost(message, repeated, tag = "5")]
    pub place_insights: ::prost::alloc::vec::Vec<PlaceInsight>,
}
/// Holds information about a place
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceInsight {
    /// The resource name of a place. This resource name can be used to retrieve
    /// details about the place using the [Places
    /// API](<https://developers.google.com/maps/documentation/places/web-service/reference/rest/v1/places/get>).
    #[prost(string, tag = "1")]
    pub place: ::prost::alloc::string::String,
}
/// Filters for the ComputeInsights RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// Required. Restricts results to places which are located in the area
    /// specified by location filters.
    #[prost(message, optional, tag = "1")]
    pub location_filter: ::core::option::Option<LocationFilter>,
    /// Required. Place type filters.
    #[prost(message, optional, tag = "2")]
    pub type_filter: ::core::option::Option<TypeFilter>,
    /// Optional. Restricts results to places whose operating status is included on
    /// this list. If operating_status is not set, OPERATING_STATUS_OPERATIONAL is
    /// used as default.
    #[prost(enumeration = "OperatingStatus", repeated, packed = "false", tag = "3")]
    pub operating_status: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Restricts results to places whose price level is included on this
    /// list. If price_level is not set, all price levels are included in the
    /// results.
    #[prost(enumeration = "PriceLevel", repeated, packed = "false", tag = "4")]
    pub price_levels: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Restricts results to places whose average user ratings are in the
    /// range specified by rating_filter. If rating_filter is not set, all ratings
    /// are included in the result.
    #[prost(message, optional, tag = "5")]
    pub rating_filter: ::core::option::Option<RatingFilter>,
}
/// Location filters.
///
/// Specifies the area of interest for the insight.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFilter {
    /// One of the following must be specified.
    #[prost(oneof = "location_filter::Area", tags = "1, 2, 3")]
    pub area: ::core::option::Option<location_filter::Area>,
}
/// Nested message and enum types in `LocationFilter`.
pub mod location_filter {
    /// A circle is defined by a center point and radius in meters.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Circle {
        /// Optional. The radius of the circle in meters
        #[prost(int32, tag = "3")]
        pub radius: i32,
        /// The center of the circle.
        #[prost(oneof = "circle::Center", tags = "1, 2")]
        pub center: ::core::option::Option<circle::Center>,
    }
    /// Nested message and enum types in `Circle`.
    pub mod circle {
        /// The center of the circle.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Center {
            /// The latitude and longitude of the center of the circle.
            #[prost(message, tag = "1")]
            LatLng(super::super::super::super::super::r#type::LatLng),
            /// The Place resource name of the center of the circle. Only point places
            /// are supported.
            #[prost(string, tag = "2")]
            Place(::prost::alloc::string::String),
        }
    }
    /// A region is a geographic boundary such as: cities, postal codes, counties,
    /// states, etc.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Region {
        /// The resource name of a region.
        #[prost(oneof = "region::Region", tags = "1")]
        pub region: ::core::option::Option<region::Region>,
    }
    /// Nested message and enum types in `Region`.
    pub mod region {
        /// The resource name of a region.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Region {
            /// The Place resource name of a region.
            #[prost(string, tag = "1")]
            Place(::prost::alloc::string::String),
        }
    }
    /// Custom Area.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomArea {
        /// Required. The custom area represented as a polygon
        #[prost(message, optional, tag = "1")]
        pub polygon: ::core::option::Option<custom_area::Polygon>,
    }
    /// Nested message and enum types in `CustomArea`.
    pub mod custom_area {
        /// A polygon is represented by a series of connected coordinates in an
        /// counterclockwise ordered sequence. The coordinates form a closed loop and
        /// define a filled region. The first and last coordinates are equivalent,
        /// and they must contain identical values. The format is a simplified
        /// version of GeoJSON polygons (we only support one counterclockwise
        /// exterior ring).
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Polygon {
            /// Optional. The coordinates that define the polygon.
            #[prost(message, repeated, tag = "1")]
            pub coordinates: ::prost::alloc::vec::Vec<
                super::super::super::super::super::r#type::LatLng,
            >,
        }
    }
    /// One of the following must be specified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Area {
        /// Area as a circle.
        #[prost(message, tag = "1")]
        Circle(Circle),
        /// Area as region.
        #[prost(message, tag = "2")]
        Region(Region),
        /// Custom area specified by a polygon.
        #[prost(message, tag = "3")]
        CustomArea(CustomArea),
    }
}
/// Place type filters.
///
/// Only Place types from
/// [Table
/// a](<https://developers.google.com/maps/documentation/places/web-service/place-types#table-a>)
/// are supported.
///
/// A place can only have a single primary type associated with it. For example,
/// the primary type might be "mexican_restaurant" or "steak_house". Use
/// included_primary_types and excluded_primary_types to filter the results on a
/// place's primary type.
///
/// A place can also have multiple type values associated with it. For example a
/// restaurant might have the following types: "seafood_restaurant",
/// "restaurant", "food", "point_of_interest", "establishment". Use
/// included_types and excluded_types to filter the results on the list of types
/// associated with a place.
///
/// If a search is specified with multiple type restrictions, only places that
/// satisfy all of the restrictions are returned. For example, if you specify
/// {"included_types": \["restaurant"\], "excluded_primary_types":
/// \["steak_house"\]}, the returned places provide "restaurant" related services
/// but do not operate primarily as a "steak_house".
///
/// If there are any conflicting types, i.e. a type appears in both
/// included_types and excluded_types types or included_primary_types and
/// excluded_primary_types, an INVALID_ARGUMENT error is returned.
///
/// One of included_types or included_primary_types must be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeFilter {
    /// Optional. Included Place types.
    #[prost(string, repeated, tag = "1")]
    pub included_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Excluded Place types.
    #[prost(string, repeated, tag = "2")]
    pub excluded_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Included primary Place types.
    #[prost(string, repeated, tag = "3")]
    pub included_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Excluded primary Place types.
    #[prost(string, repeated, tag = "4")]
    pub excluded_primary_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Average user rating filters.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RatingFilter {
    /// Optional. Restricts results to places whose average user rating is greater
    /// than or equal to min_rating. Values must be between 1.0 and 5.0.
    #[prost(float, optional, tag = "5")]
    pub min_rating: ::core::option::Option<f32>,
    /// Optional. Restricts results to places whose average user rating is strictly
    /// less than or equal to max_rating. Values must be between 1.0 and 5.0.
    #[prost(float, optional, tag = "6")]
    pub max_rating: ::core::option::Option<f32>,
}
/// Supported insights.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Insight {
    /// Not Specified.
    Unspecified = 0,
    /// Count insight.
    ///
    /// When this insight is specified ComputeInsights returns the number of
    /// places that match the specified filter criteria.
    /// ```
    /// For example if the request is:
    /// ComputeInsightsRequest {
    ///    insights: INSIGHT_COUNT
    ///    filter {
    ///      location_filter {region: <PlaceId of state of CA>}
    ///      type_filter {included_types: "restaurant"}
    ///      operating_status: OPERATING_STATUS_OPERATIONAL
    ///      price_levels: PRICE_LEVEL_FREE
    ///      price_levels: PRICE_LEVEL_INEXPENSIVE
    ///      min_rating: 4.0
    ///    }
    /// }
    ///
    /// The method will return the count of restaurants in California that are
    /// operational, with price level free or inexpensive and have an average
    /// rating of at least 4 starts.
    ///
    /// Example response:
    /// ComputeInsightsResponse {
    ///    count: <number of places>
    /// }
    /// ```
    Count = 1,
    /// Return Places
    ///
    /// When this insight is specified ComputeInsights returns Places
    /// that match the specified filter criteria.
    /// ```
    /// For example if the request is:
    /// ComputeInsightsRequest {
    ///    insights: INSIGHT_PLACES
    ///    filter {
    ///      location_filter {region: <PlaceId of state of CA>}
    ///      type_filter {included_types: "restaurant"}
    ///      operating_status: OPERATING_STATUS_OPERATIONAL
    ///      price_levels: PRICE_LEVEL_FREE
    ///      price_levels: PRICE_LEVEL_INEXPENSIVE
    ///      min_rating: 4.0
    ///    }
    /// }
    ///
    /// The method will return list of places of restaurants in
    /// California that are operational, with price level free or inexpensive and
    /// have an average rating of at least 4 stars.
    ///
    /// Example response:
    /// ComputeInsightsResponse {
    ///    place_insights { place: "places/ABC" }
    ///    place_insights { place: "places/PQR" }
    ///    place_insights { place: "places/XYZ" }
    /// }
    /// ```
    Places = 2,
}
impl Insight {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "INSIGHT_UNSPECIFIED",
            Self::Count => "INSIGHT_COUNT",
            Self::Places => "INSIGHT_PLACES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSIGHT_UNSPECIFIED" => Some(Self::Unspecified),
            "INSIGHT_COUNT" => Some(Self::Count),
            "INSIGHT_PLACES" => Some(Self::Places),
            _ => None,
        }
    }
}
/// Operating status of the place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatingStatus {
    /// Not Specified.
    Unspecified = 0,
    /// The place is operational and its open during its defined hours.
    Operational = 1,
    /// The Place is no longer in business.
    PermanentlyClosed = 3,
    /// The Place is temporarily closed and expected to reopen in the future.
    TemporarilyClosed = 4,
}
impl OperatingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "OPERATING_STATUS_UNSPECIFIED",
            Self::Operational => "OPERATING_STATUS_OPERATIONAL",
            Self::PermanentlyClosed => "OPERATING_STATUS_PERMANENTLY_CLOSED",
            Self::TemporarilyClosed => "OPERATING_STATUS_TEMPORARILY_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATING_STATUS_OPERATIONAL" => Some(Self::Operational),
            "OPERATING_STATUS_PERMANENTLY_CLOSED" => Some(Self::PermanentlyClosed),
            "OPERATING_STATUS_TEMPORARILY_CLOSED" => Some(Self::TemporarilyClosed),
            _ => None,
        }
    }
}
/// Price level of the place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceLevel {
    /// Place price level is unspecified or unknown.
    Unspecified = 0,
    /// Place provides free services.
    Free = 1,
    /// Place provides inexpensive services.
    Inexpensive = 2,
    /// Place provides moderately priced services.
    Moderate = 3,
    /// Place provides expensive services.
    Expensive = 4,
    /// Place provides very expensive services.
    VeryExpensive = 5,
}
impl PriceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "PRICE_LEVEL_UNSPECIFIED",
            Self::Free => "PRICE_LEVEL_FREE",
            Self::Inexpensive => "PRICE_LEVEL_INEXPENSIVE",
            Self::Moderate => "PRICE_LEVEL_MODERATE",
            Self::Expensive => "PRICE_LEVEL_EXPENSIVE",
            Self::VeryExpensive => "PRICE_LEVEL_VERY_EXPENSIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_LEVEL_FREE" => Some(Self::Free),
            "PRICE_LEVEL_INEXPENSIVE" => Some(Self::Inexpensive),
            "PRICE_LEVEL_MODERATE" => Some(Self::Moderate),
            "PRICE_LEVEL_EXPENSIVE" => Some(Self::Expensive),
            "PRICE_LEVEL_VERY_EXPENSIVE" => Some(Self::VeryExpensive),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod area_insights_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Places Insights API.
    #[derive(Debug, Clone)]
    pub struct AreaInsightsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AreaInsightsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AreaInsightsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AreaInsightsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AreaInsightsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Compute Insights RPC
        ///
        /// This method lets you retrieve insights about areas using a variaty of
        /// filter such as: area, place type, operating status, price level
        /// and ratings. Currently "count" and "places" insights are supported. With
        /// "count" insights you can answer questions such as "How many restaurant are
        /// located in California that are operational, are inexpensive and have an
        /// average rating of at least 4 stars" (see `insight` enum for more details).
        /// With "places" insights, you can determine which places match the
        /// requested filter. Clients can then use those place resource names to fetch
        /// more details about each individual place using the Places API.
        pub async fn compute_insights(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeInsightsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ComputeInsightsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.areainsights.v1.AreaInsights/ComputeInsights",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.areainsights.v1.AreaInsights",
                        "ComputeInsights",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
