// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_CLUSTER_MANAGER_LIST_CLUSTERS: ::grpcio::Method<super::cluster_service::ListClustersRequest, super::cluster_service::ListClustersResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/ListClusters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_GET_CLUSTER: ::grpcio::Method<super::cluster_service::GetClusterRequest, super::cluster_service::Cluster> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/GetCluster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_CREATE_CLUSTER: ::grpcio::Method<super::cluster_service::CreateClusterRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/CreateCluster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_UPDATE_CLUSTER: ::grpcio::Method<super::cluster_service::UpdateClusterRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/UpdateCluster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_UPDATE_NODE_POOL: ::grpcio::Method<super::cluster_service::UpdateNodePoolRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/UpdateNodePool",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_NODE_POOL_AUTOSCALING: ::grpcio::Method<super::cluster_service::SetNodePoolAutoscalingRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetNodePoolAutoscaling",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_LOGGING_SERVICE: ::grpcio::Method<super::cluster_service::SetLoggingServiceRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetLoggingService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_MONITORING_SERVICE: ::grpcio::Method<super::cluster_service::SetMonitoringServiceRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetMonitoringService",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_ADDONS_CONFIG: ::grpcio::Method<super::cluster_service::SetAddonsConfigRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetAddonsConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_LOCATIONS: ::grpcio::Method<super::cluster_service::SetLocationsRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetLocations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_UPDATE_MASTER: ::grpcio::Method<super::cluster_service::UpdateMasterRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/UpdateMaster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_MASTER_AUTH: ::grpcio::Method<super::cluster_service::SetMasterAuthRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetMasterAuth",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_DELETE_CLUSTER: ::grpcio::Method<super::cluster_service::DeleteClusterRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/DeleteCluster",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_LIST_OPERATIONS: ::grpcio::Method<super::cluster_service::ListOperationsRequest, super::cluster_service::ListOperationsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/ListOperations",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_GET_OPERATION: ::grpcio::Method<super::cluster_service::GetOperationRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/GetOperation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_CANCEL_OPERATION: ::grpcio::Method<super::cluster_service::CancelOperationRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/CancelOperation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_GET_SERVER_CONFIG: ::grpcio::Method<super::cluster_service::GetServerConfigRequest, super::cluster_service::ServerConfig> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/GetServerConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_LIST_NODE_POOLS: ::grpcio::Method<super::cluster_service::ListNodePoolsRequest, super::cluster_service::ListNodePoolsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/ListNodePools",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_GET_NODE_POOL: ::grpcio::Method<super::cluster_service::GetNodePoolRequest, super::cluster_service::NodePool> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/GetNodePool",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_CREATE_NODE_POOL: ::grpcio::Method<super::cluster_service::CreateNodePoolRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/CreateNodePool",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_DELETE_NODE_POOL: ::grpcio::Method<super::cluster_service::DeleteNodePoolRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/DeleteNodePool",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_ROLLBACK_NODE_POOL_UPGRADE: ::grpcio::Method<super::cluster_service::RollbackNodePoolUpgradeRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/RollbackNodePoolUpgrade",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_NODE_POOL_MANAGEMENT: ::grpcio::Method<super::cluster_service::SetNodePoolManagementRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetNodePoolManagement",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_LABELS: ::grpcio::Method<super::cluster_service::SetLabelsRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetLabels",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_LEGACY_ABAC: ::grpcio::Method<super::cluster_service::SetLegacyAbacRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetLegacyAbac",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_START_IP_ROTATION: ::grpcio::Method<super::cluster_service::StartIPRotationRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/StartIPRotation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_COMPLETE_IP_ROTATION: ::grpcio::Method<super::cluster_service::CompleteIPRotationRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/CompleteIPRotation",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_NODE_POOL_SIZE: ::grpcio::Method<super::cluster_service::SetNodePoolSizeRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetNodePoolSize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_NETWORK_POLICY: ::grpcio::Method<super::cluster_service::SetNetworkPolicyRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetNetworkPolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_SET_MAINTENANCE_POLICY: ::grpcio::Method<super::cluster_service::SetMaintenancePolicyRequest, super::cluster_service::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/SetMaintenancePolicy",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_MANAGER_LIST_USABLE_SUBNETWORKS: ::grpcio::Method<super::cluster_service::ListUsableSubnetworksRequest, super::cluster_service::ListUsableSubnetworksResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.container.v1.ClusterManager/ListUsableSubnetworks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ClusterManagerClient {
    client: ::grpcio::Client,
}

impl ClusterManagerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ClusterManagerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_clusters_opt(&self, req: &super::cluster_service::ListClustersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::ListClustersResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_LIST_CLUSTERS, req, opt)
    }

    pub fn list_clusters(&self, req: &super::cluster_service::ListClustersRequest) -> ::grpcio::Result<super::cluster_service::ListClustersResponse> {
        self.list_clusters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_clusters_async_opt(&self, req: &super::cluster_service::ListClustersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListClustersResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_LIST_CLUSTERS, req, opt)
    }

    pub fn list_clusters_async(&self, req: &super::cluster_service::ListClustersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListClustersResponse>> {
        self.list_clusters_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cluster_opt(&self, req: &super::cluster_service::GetClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Cluster> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_GET_CLUSTER, req, opt)
    }

    pub fn get_cluster(&self, req: &super::cluster_service::GetClusterRequest) -> ::grpcio::Result<super::cluster_service::Cluster> {
        self.get_cluster_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cluster_async_opt(&self, req: &super::cluster_service::GetClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Cluster>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_GET_CLUSTER, req, opt)
    }

    pub fn get_cluster_async(&self, req: &super::cluster_service::GetClusterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Cluster>> {
        self.get_cluster_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_cluster_opt(&self, req: &super::cluster_service::CreateClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_CREATE_CLUSTER, req, opt)
    }

    pub fn create_cluster(&self, req: &super::cluster_service::CreateClusterRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.create_cluster_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_cluster_async_opt(&self, req: &super::cluster_service::CreateClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_CREATE_CLUSTER, req, opt)
    }

    pub fn create_cluster_async(&self, req: &super::cluster_service::CreateClusterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.create_cluster_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_cluster_opt(&self, req: &super::cluster_service::UpdateClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_UPDATE_CLUSTER, req, opt)
    }

    pub fn update_cluster(&self, req: &super::cluster_service::UpdateClusterRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.update_cluster_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_cluster_async_opt(&self, req: &super::cluster_service::UpdateClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_UPDATE_CLUSTER, req, opt)
    }

    pub fn update_cluster_async(&self, req: &super::cluster_service::UpdateClusterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.update_cluster_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_node_pool_opt(&self, req: &super::cluster_service::UpdateNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_UPDATE_NODE_POOL, req, opt)
    }

    pub fn update_node_pool(&self, req: &super::cluster_service::UpdateNodePoolRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.update_node_pool_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_node_pool_async_opt(&self, req: &super::cluster_service::UpdateNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_UPDATE_NODE_POOL, req, opt)
    }

    pub fn update_node_pool_async(&self, req: &super::cluster_service::UpdateNodePoolRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.update_node_pool_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_autoscaling_opt(&self, req: &super::cluster_service::SetNodePoolAutoscalingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_AUTOSCALING, req, opt)
    }

    pub fn set_node_pool_autoscaling(&self, req: &super::cluster_service::SetNodePoolAutoscalingRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_node_pool_autoscaling_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_autoscaling_async_opt(&self, req: &super::cluster_service::SetNodePoolAutoscalingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_AUTOSCALING, req, opt)
    }

    pub fn set_node_pool_autoscaling_async(&self, req: &super::cluster_service::SetNodePoolAutoscalingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_node_pool_autoscaling_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_logging_service_opt(&self, req: &super::cluster_service::SetLoggingServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_LOGGING_SERVICE, req, opt)
    }

    pub fn set_logging_service(&self, req: &super::cluster_service::SetLoggingServiceRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_logging_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_logging_service_async_opt(&self, req: &super::cluster_service::SetLoggingServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_LOGGING_SERVICE, req, opt)
    }

    pub fn set_logging_service_async(&self, req: &super::cluster_service::SetLoggingServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_logging_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_monitoring_service_opt(&self, req: &super::cluster_service::SetMonitoringServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_MONITORING_SERVICE, req, opt)
    }

    pub fn set_monitoring_service(&self, req: &super::cluster_service::SetMonitoringServiceRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_monitoring_service_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_monitoring_service_async_opt(&self, req: &super::cluster_service::SetMonitoringServiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_MONITORING_SERVICE, req, opt)
    }

    pub fn set_monitoring_service_async(&self, req: &super::cluster_service::SetMonitoringServiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_monitoring_service_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_addons_config_opt(&self, req: &super::cluster_service::SetAddonsConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_ADDONS_CONFIG, req, opt)
    }

    pub fn set_addons_config(&self, req: &super::cluster_service::SetAddonsConfigRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_addons_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_addons_config_async_opt(&self, req: &super::cluster_service::SetAddonsConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_ADDONS_CONFIG, req, opt)
    }

    pub fn set_addons_config_async(&self, req: &super::cluster_service::SetAddonsConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_addons_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_locations_opt(&self, req: &super::cluster_service::SetLocationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_LOCATIONS, req, opt)
    }

    pub fn set_locations(&self, req: &super::cluster_service::SetLocationsRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_locations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_locations_async_opt(&self, req: &super::cluster_service::SetLocationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_LOCATIONS, req, opt)
    }

    pub fn set_locations_async(&self, req: &super::cluster_service::SetLocationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_locations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_master_opt(&self, req: &super::cluster_service::UpdateMasterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_UPDATE_MASTER, req, opt)
    }

    pub fn update_master(&self, req: &super::cluster_service::UpdateMasterRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.update_master_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_master_async_opt(&self, req: &super::cluster_service::UpdateMasterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_UPDATE_MASTER, req, opt)
    }

    pub fn update_master_async(&self, req: &super::cluster_service::UpdateMasterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.update_master_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_master_auth_opt(&self, req: &super::cluster_service::SetMasterAuthRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_MASTER_AUTH, req, opt)
    }

    pub fn set_master_auth(&self, req: &super::cluster_service::SetMasterAuthRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_master_auth_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_master_auth_async_opt(&self, req: &super::cluster_service::SetMasterAuthRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_MASTER_AUTH, req, opt)
    }

    pub fn set_master_auth_async(&self, req: &super::cluster_service::SetMasterAuthRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_master_auth_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_cluster_opt(&self, req: &super::cluster_service::DeleteClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_DELETE_CLUSTER, req, opt)
    }

    pub fn delete_cluster(&self, req: &super::cluster_service::DeleteClusterRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.delete_cluster_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_cluster_async_opt(&self, req: &super::cluster_service::DeleteClusterRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_DELETE_CLUSTER, req, opt)
    }

    pub fn delete_cluster_async(&self, req: &super::cluster_service::DeleteClusterRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.delete_cluster_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_operations_opt(&self, req: &super::cluster_service::ListOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::ListOperationsResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_LIST_OPERATIONS, req, opt)
    }

    pub fn list_operations(&self, req: &super::cluster_service::ListOperationsRequest) -> ::grpcio::Result<super::cluster_service::ListOperationsResponse> {
        self.list_operations_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_operations_async_opt(&self, req: &super::cluster_service::ListOperationsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListOperationsResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_LIST_OPERATIONS, req, opt)
    }

    pub fn list_operations_async(&self, req: &super::cluster_service::ListOperationsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListOperationsResponse>> {
        self.list_operations_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_operation_opt(&self, req: &super::cluster_service::GetOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_GET_OPERATION, req, opt)
    }

    pub fn get_operation(&self, req: &super::cluster_service::GetOperationRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.get_operation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_operation_async_opt(&self, req: &super::cluster_service::GetOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_GET_OPERATION, req, opt)
    }

    pub fn get_operation_async(&self, req: &super::cluster_service::GetOperationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.get_operation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_operation_opt(&self, req: &super::cluster_service::CancelOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_CANCEL_OPERATION, req, opt)
    }

    pub fn cancel_operation(&self, req: &super::cluster_service::CancelOperationRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.cancel_operation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_operation_async_opt(&self, req: &super::cluster_service::CancelOperationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_CANCEL_OPERATION, req, opt)
    }

    pub fn cancel_operation_async(&self, req: &super::cluster_service::CancelOperationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.cancel_operation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_server_config_opt(&self, req: &super::cluster_service::GetServerConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::ServerConfig> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_GET_SERVER_CONFIG, req, opt)
    }

    pub fn get_server_config(&self, req: &super::cluster_service::GetServerConfigRequest) -> ::grpcio::Result<super::cluster_service::ServerConfig> {
        self.get_server_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_server_config_async_opt(&self, req: &super::cluster_service::GetServerConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ServerConfig>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_GET_SERVER_CONFIG, req, opt)
    }

    pub fn get_server_config_async(&self, req: &super::cluster_service::GetServerConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ServerConfig>> {
        self.get_server_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_node_pools_opt(&self, req: &super::cluster_service::ListNodePoolsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::ListNodePoolsResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_LIST_NODE_POOLS, req, opt)
    }

    pub fn list_node_pools(&self, req: &super::cluster_service::ListNodePoolsRequest) -> ::grpcio::Result<super::cluster_service::ListNodePoolsResponse> {
        self.list_node_pools_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_node_pools_async_opt(&self, req: &super::cluster_service::ListNodePoolsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListNodePoolsResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_LIST_NODE_POOLS, req, opt)
    }

    pub fn list_node_pools_async(&self, req: &super::cluster_service::ListNodePoolsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListNodePoolsResponse>> {
        self.list_node_pools_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_pool_opt(&self, req: &super::cluster_service::GetNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::NodePool> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_GET_NODE_POOL, req, opt)
    }

    pub fn get_node_pool(&self, req: &super::cluster_service::GetNodePoolRequest) -> ::grpcio::Result<super::cluster_service::NodePool> {
        self.get_node_pool_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_node_pool_async_opt(&self, req: &super::cluster_service::GetNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::NodePool>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_GET_NODE_POOL, req, opt)
    }

    pub fn get_node_pool_async(&self, req: &super::cluster_service::GetNodePoolRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::NodePool>> {
        self.get_node_pool_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_node_pool_opt(&self, req: &super::cluster_service::CreateNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_CREATE_NODE_POOL, req, opt)
    }

    pub fn create_node_pool(&self, req: &super::cluster_service::CreateNodePoolRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.create_node_pool_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_node_pool_async_opt(&self, req: &super::cluster_service::CreateNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_CREATE_NODE_POOL, req, opt)
    }

    pub fn create_node_pool_async(&self, req: &super::cluster_service::CreateNodePoolRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.create_node_pool_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_node_pool_opt(&self, req: &super::cluster_service::DeleteNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_DELETE_NODE_POOL, req, opt)
    }

    pub fn delete_node_pool(&self, req: &super::cluster_service::DeleteNodePoolRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.delete_node_pool_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_node_pool_async_opt(&self, req: &super::cluster_service::DeleteNodePoolRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_DELETE_NODE_POOL, req, opt)
    }

    pub fn delete_node_pool_async(&self, req: &super::cluster_service::DeleteNodePoolRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.delete_node_pool_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rollback_node_pool_upgrade_opt(&self, req: &super::cluster_service::RollbackNodePoolUpgradeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_ROLLBACK_NODE_POOL_UPGRADE, req, opt)
    }

    pub fn rollback_node_pool_upgrade(&self, req: &super::cluster_service::RollbackNodePoolUpgradeRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.rollback_node_pool_upgrade_opt(req, ::grpcio::CallOption::default())
    }

    pub fn rollback_node_pool_upgrade_async_opt(&self, req: &super::cluster_service::RollbackNodePoolUpgradeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_ROLLBACK_NODE_POOL_UPGRADE, req, opt)
    }

    pub fn rollback_node_pool_upgrade_async(&self, req: &super::cluster_service::RollbackNodePoolUpgradeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.rollback_node_pool_upgrade_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_management_opt(&self, req: &super::cluster_service::SetNodePoolManagementRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_MANAGEMENT, req, opt)
    }

    pub fn set_node_pool_management(&self, req: &super::cluster_service::SetNodePoolManagementRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_node_pool_management_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_management_async_opt(&self, req: &super::cluster_service::SetNodePoolManagementRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_MANAGEMENT, req, opt)
    }

    pub fn set_node_pool_management_async(&self, req: &super::cluster_service::SetNodePoolManagementRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_node_pool_management_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_labels_opt(&self, req: &super::cluster_service::SetLabelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_LABELS, req, opt)
    }

    pub fn set_labels(&self, req: &super::cluster_service::SetLabelsRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_labels_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_labels_async_opt(&self, req: &super::cluster_service::SetLabelsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_LABELS, req, opt)
    }

    pub fn set_labels_async(&self, req: &super::cluster_service::SetLabelsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_labels_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_legacy_abac_opt(&self, req: &super::cluster_service::SetLegacyAbacRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_LEGACY_ABAC, req, opt)
    }

    pub fn set_legacy_abac(&self, req: &super::cluster_service::SetLegacyAbacRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_legacy_abac_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_legacy_abac_async_opt(&self, req: &super::cluster_service::SetLegacyAbacRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_LEGACY_ABAC, req, opt)
    }

    pub fn set_legacy_abac_async(&self, req: &super::cluster_service::SetLegacyAbacRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_legacy_abac_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_ip_rotation_opt(&self, req: &super::cluster_service::StartIPRotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_START_IP_ROTATION, req, opt)
    }

    pub fn start_ip_rotation(&self, req: &super::cluster_service::StartIPRotationRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.start_ip_rotation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn start_ip_rotation_async_opt(&self, req: &super::cluster_service::StartIPRotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_START_IP_ROTATION, req, opt)
    }

    pub fn start_ip_rotation_async(&self, req: &super::cluster_service::StartIPRotationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.start_ip_rotation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn complete_ip_rotation_opt(&self, req: &super::cluster_service::CompleteIPRotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_COMPLETE_IP_ROTATION, req, opt)
    }

    pub fn complete_ip_rotation(&self, req: &super::cluster_service::CompleteIPRotationRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.complete_ip_rotation_opt(req, ::grpcio::CallOption::default())
    }

    pub fn complete_ip_rotation_async_opt(&self, req: &super::cluster_service::CompleteIPRotationRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_COMPLETE_IP_ROTATION, req, opt)
    }

    pub fn complete_ip_rotation_async(&self, req: &super::cluster_service::CompleteIPRotationRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.complete_ip_rotation_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_size_opt(&self, req: &super::cluster_service::SetNodePoolSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_SIZE, req, opt)
    }

    pub fn set_node_pool_size(&self, req: &super::cluster_service::SetNodePoolSizeRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_node_pool_size_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_node_pool_size_async_opt(&self, req: &super::cluster_service::SetNodePoolSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_SIZE, req, opt)
    }

    pub fn set_node_pool_size_async(&self, req: &super::cluster_service::SetNodePoolSizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_node_pool_size_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_network_policy_opt(&self, req: &super::cluster_service::SetNetworkPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_NETWORK_POLICY, req, opt)
    }

    pub fn set_network_policy(&self, req: &super::cluster_service::SetNetworkPolicyRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_network_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_network_policy_async_opt(&self, req: &super::cluster_service::SetNetworkPolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_NETWORK_POLICY, req, opt)
    }

    pub fn set_network_policy_async(&self, req: &super::cluster_service::SetNetworkPolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_network_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_maintenance_policy_opt(&self, req: &super::cluster_service::SetMaintenancePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_SET_MAINTENANCE_POLICY, req, opt)
    }

    pub fn set_maintenance_policy(&self, req: &super::cluster_service::SetMaintenancePolicyRequest) -> ::grpcio::Result<super::cluster_service::Operation> {
        self.set_maintenance_policy_opt(req, ::grpcio::CallOption::default())
    }

    pub fn set_maintenance_policy_async_opt(&self, req: &super::cluster_service::SetMaintenancePolicyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_SET_MAINTENANCE_POLICY, req, opt)
    }

    pub fn set_maintenance_policy_async(&self, req: &super::cluster_service::SetMaintenancePolicyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::Operation>> {
        self.set_maintenance_policy_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_usable_subnetworks_opt(&self, req: &super::cluster_service::ListUsableSubnetworksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cluster_service::ListUsableSubnetworksResponse> {
        self.client.unary_call(&METHOD_CLUSTER_MANAGER_LIST_USABLE_SUBNETWORKS, req, opt)
    }

    pub fn list_usable_subnetworks(&self, req: &super::cluster_service::ListUsableSubnetworksRequest) -> ::grpcio::Result<super::cluster_service::ListUsableSubnetworksResponse> {
        self.list_usable_subnetworks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_usable_subnetworks_async_opt(&self, req: &super::cluster_service::ListUsableSubnetworksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListUsableSubnetworksResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_MANAGER_LIST_USABLE_SUBNETWORKS, req, opt)
    }

    pub fn list_usable_subnetworks_async(&self, req: &super::cluster_service::ListUsableSubnetworksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cluster_service::ListUsableSubnetworksResponse>> {
        self.list_usable_subnetworks_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ClusterManager {
    fn list_clusters(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::ListClustersRequest, sink: ::grpcio::UnarySink<super::cluster_service::ListClustersResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_cluster(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::GetClusterRequest, sink: ::grpcio::UnarySink<super::cluster_service::Cluster>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_cluster(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::CreateClusterRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_cluster(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::UpdateClusterRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_node_pool(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::UpdateNodePoolRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_node_pool_autoscaling(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetNodePoolAutoscalingRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_logging_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetLoggingServiceRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_monitoring_service(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetMonitoringServiceRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_addons_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetAddonsConfigRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_locations(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetLocationsRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_master(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::UpdateMasterRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_master_auth(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetMasterAuthRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_cluster(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::DeleteClusterRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_operations(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::ListOperationsRequest, sink: ::grpcio::UnarySink<super::cluster_service::ListOperationsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_operation(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::GetOperationRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_operation(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::CancelOperationRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_server_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::GetServerConfigRequest, sink: ::grpcio::UnarySink<super::cluster_service::ServerConfig>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_node_pools(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::ListNodePoolsRequest, sink: ::grpcio::UnarySink<super::cluster_service::ListNodePoolsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_node_pool(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::GetNodePoolRequest, sink: ::grpcio::UnarySink<super::cluster_service::NodePool>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_node_pool(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::CreateNodePoolRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_node_pool(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::DeleteNodePoolRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn rollback_node_pool_upgrade(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::RollbackNodePoolUpgradeRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_node_pool_management(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetNodePoolManagementRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_labels(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetLabelsRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_legacy_abac(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetLegacyAbacRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn start_ip_rotation(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::StartIPRotationRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn complete_ip_rotation(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::CompleteIPRotationRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_node_pool_size(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetNodePoolSizeRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_network_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetNetworkPolicyRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn set_maintenance_policy(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::SetMaintenancePolicyRequest, sink: ::grpcio::UnarySink<super::cluster_service::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_usable_subnetworks(&mut self, ctx: ::grpcio::RpcContext, _req: super::cluster_service::ListUsableSubnetworksRequest, sink: ::grpcio::UnarySink<super::cluster_service::ListUsableSubnetworksResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_cluster_manager<S: ClusterManager + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_LIST_CLUSTERS, move |ctx, req, resp| {
        instance.list_clusters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_GET_CLUSTER, move |ctx, req, resp| {
        instance.get_cluster(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_CREATE_CLUSTER, move |ctx, req, resp| {
        instance.create_cluster(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_UPDATE_CLUSTER, move |ctx, req, resp| {
        instance.update_cluster(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_UPDATE_NODE_POOL, move |ctx, req, resp| {
        instance.update_node_pool(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_AUTOSCALING, move |ctx, req, resp| {
        instance.set_node_pool_autoscaling(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_LOGGING_SERVICE, move |ctx, req, resp| {
        instance.set_logging_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_MONITORING_SERVICE, move |ctx, req, resp| {
        instance.set_monitoring_service(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_ADDONS_CONFIG, move |ctx, req, resp| {
        instance.set_addons_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_LOCATIONS, move |ctx, req, resp| {
        instance.set_locations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_UPDATE_MASTER, move |ctx, req, resp| {
        instance.update_master(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_MASTER_AUTH, move |ctx, req, resp| {
        instance.set_master_auth(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_DELETE_CLUSTER, move |ctx, req, resp| {
        instance.delete_cluster(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_LIST_OPERATIONS, move |ctx, req, resp| {
        instance.list_operations(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_GET_OPERATION, move |ctx, req, resp| {
        instance.get_operation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_CANCEL_OPERATION, move |ctx, req, resp| {
        instance.cancel_operation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_GET_SERVER_CONFIG, move |ctx, req, resp| {
        instance.get_server_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_LIST_NODE_POOLS, move |ctx, req, resp| {
        instance.list_node_pools(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_GET_NODE_POOL, move |ctx, req, resp| {
        instance.get_node_pool(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_CREATE_NODE_POOL, move |ctx, req, resp| {
        instance.create_node_pool(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_DELETE_NODE_POOL, move |ctx, req, resp| {
        instance.delete_node_pool(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_ROLLBACK_NODE_POOL_UPGRADE, move |ctx, req, resp| {
        instance.rollback_node_pool_upgrade(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_MANAGEMENT, move |ctx, req, resp| {
        instance.set_node_pool_management(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_LABELS, move |ctx, req, resp| {
        instance.set_labels(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_LEGACY_ABAC, move |ctx, req, resp| {
        instance.set_legacy_abac(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_START_IP_ROTATION, move |ctx, req, resp| {
        instance.start_ip_rotation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_COMPLETE_IP_ROTATION, move |ctx, req, resp| {
        instance.complete_ip_rotation(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_NODE_POOL_SIZE, move |ctx, req, resp| {
        instance.set_node_pool_size(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_NETWORK_POLICY, move |ctx, req, resp| {
        instance.set_network_policy(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_SET_MAINTENANCE_POLICY, move |ctx, req, resp| {
        instance.set_maintenance_policy(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_CLUSTER_MANAGER_LIST_USABLE_SUBNETWORKS, move |ctx, req, resp| {
        instance.list_usable_subnetworks(ctx, req, resp)
    });
    builder.build()
}
