// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Underworlds {
    fn helo(&self, o: ::grpc::RequestOptions, p: super::underworlds::Name) -> ::grpc::SingleResponse<super::underworlds::Client>;

    fn uptime(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Time>;

    fn topology(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Topology>;

    fn reset(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn get_nodes_len(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Size>;

    fn get_nodes_ids(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Nodes>;

    fn get_root_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Node>;

    fn get_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Node>;

    fn update_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn delete_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn get_node_invalidations(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::StreamingResponse<super::underworlds::NodeInvalidation>;

    fn timeline_origin(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Time>;

    fn event(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn start_situation(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn end_situation(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;

    fn get_timeline_invalidations(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::StreamingResponse<super::underworlds::TimelineInvalidation>;

    fn has_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Bool>;

    fn get_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Mesh>;

    fn push_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Empty>;
}

// client

pub struct UnderworldsClient {
    grpc_client: ::grpc::Client,
    method_helo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Name, super::underworlds::Client>>,
    method_uptime: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Client, super::underworlds::Time>>,
    method_topology: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Client, super::underworlds::Topology>>,
    method_reset: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Client, super::underworlds::Empty>>,
    method_getNodesLen: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::Size>>,
    method_getNodesIds: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::Nodes>>,
    method_getRootNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::Node>>,
    method_getNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::NodeInContext, super::underworlds::Node>>,
    method_updateNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::NodeInContext, super::underworlds::Empty>>,
    method_deleteNode: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::NodeInContext, super::underworlds::Empty>>,
    method_getNodeInvalidations: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::NodeInvalidation>>,
    method_timelineOrigin: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::Time>>,
    method_event: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::SituationInContext, super::underworlds::Empty>>,
    method_startSituation: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::SituationInContext, super::underworlds::Empty>>,
    method_endSituation: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::SituationInContext, super::underworlds::Empty>>,
    method_getTimelineInvalidations: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::Context, super::underworlds::TimelineInvalidation>>,
    method_hasMesh: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::MeshInContext, super::underworlds::Bool>>,
    method_getMesh: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::MeshInContext, super::underworlds::Mesh>>,
    method_pushMesh: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::underworlds::MeshInContext, super::underworlds::Empty>>,
}

impl UnderworldsClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        UnderworldsClient {
            grpc_client: grpc_client,
            method_helo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/helo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_uptime: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/uptime".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_topology: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/topology".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_reset: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/reset".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getNodesLen: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getNodesLen".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getNodesIds: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getNodesIds".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getRootNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getRootNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_updateNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/updateNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_deleteNode: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/deleteNode".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getNodeInvalidations: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getNodeInvalidations".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_timelineOrigin: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/timelineOrigin".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_event: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/event".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_startSituation: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/startSituation".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_endSituation: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/endSituation".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getTimelineInvalidations: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getTimelineInvalidations".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_hasMesh: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/hasMesh".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_getMesh: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/getMesh".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_pushMesh: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/underworlds.Underworlds/pushMesh".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            UnderworldsClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            UnderworldsClient::with_client(c)
        })
    }
}

impl Underworlds for UnderworldsClient {
    fn helo(&self, o: ::grpc::RequestOptions, p: super::underworlds::Name) -> ::grpc::SingleResponse<super::underworlds::Client> {
        self.grpc_client.call_unary(o, p, self.method_helo.clone())
    }

    fn uptime(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Time> {
        self.grpc_client.call_unary(o, p, self.method_uptime.clone())
    }

    fn topology(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Topology> {
        self.grpc_client.call_unary(o, p, self.method_topology.clone())
    }

    fn reset(&self, o: ::grpc::RequestOptions, p: super::underworlds::Client) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_reset.clone())
    }

    fn get_nodes_len(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Size> {
        self.grpc_client.call_unary(o, p, self.method_getNodesLen.clone())
    }

    fn get_nodes_ids(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Nodes> {
        self.grpc_client.call_unary(o, p, self.method_getNodesIds.clone())
    }

    fn get_root_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Node> {
        self.grpc_client.call_unary(o, p, self.method_getRootNode.clone())
    }

    fn get_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Node> {
        self.grpc_client.call_unary(o, p, self.method_getNode.clone())
    }

    fn update_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_updateNode.clone())
    }

    fn delete_node(&self, o: ::grpc::RequestOptions, p: super::underworlds::NodeInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_deleteNode.clone())
    }

    fn get_node_invalidations(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::StreamingResponse<super::underworlds::NodeInvalidation> {
        self.grpc_client.call_server_streaming(o, p, self.method_getNodeInvalidations.clone())
    }

    fn timeline_origin(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::SingleResponse<super::underworlds::Time> {
        self.grpc_client.call_unary(o, p, self.method_timelineOrigin.clone())
    }

    fn event(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_event.clone())
    }

    fn start_situation(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_startSituation.clone())
    }

    fn end_situation(&self, o: ::grpc::RequestOptions, p: super::underworlds::SituationInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_endSituation.clone())
    }

    fn get_timeline_invalidations(&self, o: ::grpc::RequestOptions, p: super::underworlds::Context) -> ::grpc::StreamingResponse<super::underworlds::TimelineInvalidation> {
        self.grpc_client.call_server_streaming(o, p, self.method_getTimelineInvalidations.clone())
    }

    fn has_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Bool> {
        self.grpc_client.call_unary(o, p, self.method_hasMesh.clone())
    }

    fn get_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Mesh> {
        self.grpc_client.call_unary(o, p, self.method_getMesh.clone())
    }

    fn push_mesh(&self, o: ::grpc::RequestOptions, p: super::underworlds::MeshInContext) -> ::grpc::SingleResponse<super::underworlds::Empty> {
        self.grpc_client.call_unary(o, p, self.method_pushMesh.clone())
    }
}

// server

pub struct UnderworldsServer;


impl UnderworldsServer {
    pub fn new_service_def<H : Underworlds + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/underworlds.Underworlds",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/helo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.helo(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/uptime".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.uptime(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/topology".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.topology(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/reset".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.reset(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getNodesLen".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_nodes_len(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getNodesIds".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_nodes_ids(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getRootNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_root_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/updateNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/deleteNode".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_node(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getNodeInvalidations".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.get_node_invalidations(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/timelineOrigin".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.timeline_origin(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/event".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.event(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/startSituation".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.start_situation(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/endSituation".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.end_situation(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getTimelineInvalidations".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.get_timeline_invalidations(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/hasMesh".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.has_mesh(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/getMesh".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_mesh(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/underworlds.Underworlds/pushMesh".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.push_mesh(o, p))
                    },
                ),
            ],
        )
    }
}
