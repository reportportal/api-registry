#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<super::super::super::google::r#type::DateTime>,
    #[prost(oneof = "upload_request::Payload", tags = "3, 4, 5, 6")]
    pub payload: ::core::option::Option<upload_request::Payload>,
}
/// Nested message and enum types in `UploadRequest`.
pub mod upload_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "3")]
        Start(super::Start),
        #[prost(message, tag = "4")]
        Finish(super::Finish),
        #[prost(message, tag = "5")]
        SaveLog(super::SaveLog),
        #[prost(message, tag = "6")]
        SaveData(super::SaveData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Start {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(oneof = "start::Item", tags = "4, 5, 6")]
    pub item: ::core::option::Option<start::Item>,
}
/// Nested message and enum types in `Start`.
pub mod start {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "4")]
        StartRun(super::StartRun),
        #[prost(message, tag = "5")]
        StartResult(super::StartResult),
        #[prost(message, tag = "6")]
        NestedStep(super::StartNestedStep),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResult {
    #[prost(string, tag = "1")]
    pub code_ref: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub test_case_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub run_uuid: ::prost::alloc::string::String,
    #[prost(enumeration = "ResultType", tag = "4")]
    pub r#type: i32,
    #[prost(string, optional, tag = "5")]
    pub retry_of: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub parameters: ::prost::alloc::vec::Vec<Parameter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNestedStep {
    #[prost(string, tag = "1")]
    pub run_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub result_uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRun {
    #[prost(bool, tag = "1")]
    pub rerun: bool,
    #[prost(string, optional, tag = "2")]
    pub rerun_of: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "RunMode", tag = "3")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finish {
    #[prost(string, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub attributes: ::prost::alloc::vec::Vec<Attribute>,
    #[prost(oneof = "finish::Item", tags = "4, 5, 6")]
    pub item: ::core::option::Option<finish::Item>,
}
/// Nested message and enum types in `Finish`.
pub mod finish {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "4")]
        FinishRun(super::FinishRun),
        #[prost(message, tag = "5")]
        FinishResult(super::FinishResult),
        #[prost(message, tag = "6")]
        FinishNestedStep(super::FinishNestedStep),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishRun {
    #[prost(enumeration = "Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishResult {
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    #[prost(string, optional, tag = "2")]
    pub retry_of: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub issue: ::core::option::Option<Issue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishNestedStep {
    #[prost(enumeration = "Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
    #[prost(string, tag = "1")]
    pub issue_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub auto_analyzed: bool,
    #[prost(bool, tag = "4")]
    pub ignore_analyzer: bool,
    #[prost(message, repeated, tag = "5")]
    pub external_system_issues: ::prost::alloc::vec::Vec<ExternalSystemIssue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalSystemIssue {
    #[prost(string, tag = "1")]
    pub ticket_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub submit_date: ::core::option::Option<
        super::super::super::google::r#type::DateTime,
    >,
    #[prost(string, tag = "3")]
    pub bts_url: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub bts_project: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub plugin_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveLog {
    #[prost(string, tag = "1")]
    pub result_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub run_uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub level: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub file: ::core::option::Option<File>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub attachment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SaveData {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadResponse {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(enumeration = "OperationResult", tag = "2")]
    pub result: i32,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub hidden: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationResult {
    OkUnspecified = 0,
    Fail = 1,
}
impl OperationResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationResult::OkUnspecified => "OPERATION_RESULT_OK_UNSPECIFIED",
            OperationResult::Fail => "OPERATION_RESULT_FAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_RESULT_OK_UNSPECIFIED" => Some(Self::OkUnspecified),
            "OPERATION_RESULT_FAIL" => Some(Self::Fail),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResultType {
    StepUnspecified = 0,
    BeforeClass = 1,
    BeforeGroups = 2,
    BeforeMethod = 3,
    BeforeSuite = 4,
    BeforeTest = 5,
    AfterClass = 6,
    AfterGroups = 7,
    AfterMethod = 8,
    AfterSuite = 9,
    AfterTest = 10,
}
impl ResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultType::StepUnspecified => "RESULT_TYPE_STEP_UNSPECIFIED",
            ResultType::BeforeClass => "RESULT_TYPE_BEFORE_CLASS",
            ResultType::BeforeGroups => "RESULT_TYPE_BEFORE_GROUPS",
            ResultType::BeforeMethod => "RESULT_TYPE_BEFORE_METHOD",
            ResultType::BeforeSuite => "RESULT_TYPE_BEFORE_SUITE",
            ResultType::BeforeTest => "RESULT_TYPE_BEFORE_TEST",
            ResultType::AfterClass => "RESULT_TYPE_AFTER_CLASS",
            ResultType::AfterGroups => "RESULT_TYPE_AFTER_GROUPS",
            ResultType::AfterMethod => "RESULT_TYPE_AFTER_METHOD",
            ResultType::AfterSuite => "RESULT_TYPE_AFTER_SUITE",
            ResultType::AfterTest => "RESULT_TYPE_AFTER_TEST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESULT_TYPE_STEP_UNSPECIFIED" => Some(Self::StepUnspecified),
            "RESULT_TYPE_BEFORE_CLASS" => Some(Self::BeforeClass),
            "RESULT_TYPE_BEFORE_GROUPS" => Some(Self::BeforeGroups),
            "RESULT_TYPE_BEFORE_METHOD" => Some(Self::BeforeMethod),
            "RESULT_TYPE_BEFORE_SUITE" => Some(Self::BeforeSuite),
            "RESULT_TYPE_BEFORE_TEST" => Some(Self::BeforeTest),
            "RESULT_TYPE_AFTER_CLASS" => Some(Self::AfterClass),
            "RESULT_TYPE_AFTER_GROUPS" => Some(Self::AfterGroups),
            "RESULT_TYPE_AFTER_METHOD" => Some(Self::AfterMethod),
            "RESULT_TYPE_AFTER_SUITE" => Some(Self::AfterSuite),
            "RESULT_TYPE_AFTER_TEST" => Some(Self::AfterTest),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    PassedUnspecified = 0,
    Failed = 1,
    Skipped = 2,
    Stopped = 3,
    Interrupted = 4,
    Cancelled = 5,
    Info = 6,
    Warn = 7,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::PassedUnspecified => "STATUS_PASSED_UNSPECIFIED",
            Status::Failed => "STATUS_FAILED",
            Status::Skipped => "STATUS_SKIPPED",
            Status::Stopped => "STATUS_STOPPED",
            Status::Interrupted => "STATUS_INTERRUPTED",
            Status::Cancelled => "STATUS_CANCELLED",
            Status::Info => "STATUS_INFO",
            Status::Warn => "STATUS_WARN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATUS_PASSED_UNSPECIFIED" => Some(Self::PassedUnspecified),
            "STATUS_FAILED" => Some(Self::Failed),
            "STATUS_SKIPPED" => Some(Self::Skipped),
            "STATUS_STOPPED" => Some(Self::Stopped),
            "STATUS_INTERRUPTED" => Some(Self::Interrupted),
            "STATUS_CANCELLED" => Some(Self::Cancelled),
            "STATUS_INFO" => Some(Self::Info),
            "STATUS_WARN" => Some(Self::Warn),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunMode {
    DefaultUnspecified = 0,
    Debug = 1,
}
impl RunMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RunMode::DefaultUnspecified => "RUN_MODE_DEFAULT_UNSPECIFIED",
            RunMode::Debug => "RUN_MODE_DEBUG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RUN_MODE_DEFAULT_UNSPECIFIED" => Some(Self::DefaultUnspecified),
            "RUN_MODE_DEBUG" => Some(Self::Debug),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod report_portal_reporting_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ReportPortalReportingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReportPortalReportingServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReportPortalReportingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> ReportPortalReportingServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            ReportPortalReportingServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn upload(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadRequest>,
        ) -> Result<tonic::Response<super::UploadResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/reportportal.reporting.v1.ReportPortalReportingService/Upload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn upload_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UploadRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UploadResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/reportportal.reporting.v1.ReportPortalReportingService/UploadStream",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod report_portal_reporting_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ReportPortalReportingServiceServer.
    #[async_trait]
    pub trait ReportPortalReportingService: Send + Sync + 'static {
        async fn upload(
            &self,
            request: tonic::Request<super::UploadRequest>,
        ) -> Result<tonic::Response<super::UploadResponse>, tonic::Status>;
        /// Server streaming response type for the UploadStream method.
        type UploadStreamStream: futures_core::Stream<
                Item = Result<super::UploadResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn upload_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::UploadRequest>>,
        ) -> Result<tonic::Response<Self::UploadStreamStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ReportPortalReportingServiceServer<T: ReportPortalReportingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ReportPortalReportingService> ReportPortalReportingServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ReportPortalReportingServiceServer<T>
    where
        T: ReportPortalReportingService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/reportportal.reporting.v1.ReportPortalReportingService/Upload" => {
                    #[allow(non_camel_case_types)]
                    struct UploadSvc<T: ReportPortalReportingService>(pub Arc<T>);
                    impl<
                        T: ReportPortalReportingService,
                    > tonic::server::UnaryService<super::UploadRequest>
                    for UploadSvc<T> {
                        type Response = super::UploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/reportportal.reporting.v1.ReportPortalReportingService/UploadStream" => {
                    #[allow(non_camel_case_types)]
                    struct UploadStreamSvc<T: ReportPortalReportingService>(pub Arc<T>);
                    impl<
                        T: ReportPortalReportingService,
                    > tonic::server::StreamingService<super::UploadRequest>
                    for UploadStreamSvc<T> {
                        type Response = super::UploadResponse;
                        type ResponseStream = T::UploadStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::UploadRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).upload_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UploadStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ReportPortalReportingService> Clone
    for ReportPortalReportingServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ReportPortalReportingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReportPortalReportingService> tonic::server::NamedService
    for ReportPortalReportingServiceServer<T> {
        const NAME: &'static str = "reportportal.reporting.v1.ReportPortalReportingService";
    }
}
