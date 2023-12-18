/// AggregationRule describes how to locate ClusterRoles to aggregate into the ClusterRole
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregationRule {
    /// ClusterRoleSelectors holds a list of selectors which will be used to find ClusterRoles and create the rules.
    /// If any of the selectors match, then the ClusterRole's permissions will be added
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub cluster_role_selectors: ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterRole {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Rules holds all the PolicyRules for this ClusterRole
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub rules: ::prost::alloc::vec::Vec<PolicyRule>,
    /// AggregationRule is an optional field that describes how to build the Rules for this ClusterRole.
    /// If AggregationRule is set, then the Rules are controller managed and direct changes to Rules will be
    /// stomped by the controller.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub aggregation_rule: ::core::option::Option<AggregationRule>,
}
/// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace,
/// and adds who information via Subject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterRoleBinding {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Subjects holds references to the objects the role applies to.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub subjects: ::prost::alloc::vec::Vec<Subject>,
    /// RoleRef can only reference a ClusterRole in the global namespace.
    /// If the RoleRef cannot be resolved, the Authorizer must return an error.
    /// This field is immutable.
    #[prost(message, optional, tag="3")]
    pub role_ref: ::core::option::Option<RoleRef>,
}
/// ClusterRoleBindingList is a collection of ClusterRoleBindings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterRoleBindingList {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of ClusterRoleBindings
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ClusterRoleBinding>,
}
/// ClusterRoleList is a collection of ClusterRoles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterRoleList {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of ClusterRoles
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ClusterRole>,
}
/// PolicyRule holds information that describes a policy rule, but does not contain information
/// about who the rule applies to or which namespace the rule applies to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRule {
    /// Verbs is a list of Verbs that apply to ALL the ResourceKinds contained in this rule. '*' represents all verbs.
    #[prost(string, repeated, tag="1")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of
    /// the enumerated resources in any API group will be allowed. "" represents the core API group and "*" represents all API groups.
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub api_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Resources is a list of resources this rule applies to. '*' represents all resources.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full, final step in the path
    /// Since non-resource URLs are not namespaced, this field is only applicable for ClusterRoles referenced from a ClusterRoleBinding.
    /// Rules can either apply to API resources (such as "pods" or "secrets") or non-resource URL paths (such as "/api"),  but not both.
    /// +optional
    #[prost(string, repeated, tag="5")]
    pub non_resource_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Rules holds all the PolicyRules for this Role
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub rules: ::prost::alloc::vec::Vec<PolicyRule>,
}
/// RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace.
/// It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given
/// namespace only have effect in that namespace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleBinding {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Subjects holds references to the objects the role applies to.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub subjects: ::prost::alloc::vec::Vec<Subject>,
    /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace.
    /// If the RoleRef cannot be resolved, the Authorizer must return an error.
    /// This field is immutable.
    #[prost(message, optional, tag="3")]
    pub role_ref: ::core::option::Option<RoleRef>,
}
/// RoleBindingList is a collection of RoleBindings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleBindingList {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of RoleBindings
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<RoleBinding>,
}
/// RoleList is a collection of Roles
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleList {
    /// Standard object's metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of Roles
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Role>,
}
/// RoleRef contains information that points to the role being used
/// +structType=atomic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleRef {
    /// APIGroup is the group for the resource being referenced
    #[prost(string, optional, tag="1")]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the type of resource being referenced
    #[prost(string, optional, tag="2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Subject contains a reference to the object or user identities a role binding applies to.  This can either hold a direct API object reference,
/// or a value for non-objects such as user and group names.
/// +structType=atomic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// Kind of object being referenced. Values defined by this API group are "User", "Group", and "ServiceAccount".
    /// If the Authorizer does not recognized the kind value, the Authorizer should report an error.
    #[prost(string, optional, tag="1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// APIGroup holds the API group of the referenced subject.
    /// Defaults to "" for ServiceAccount subjects.
    /// Defaults to "rbac.authorization.k8s.io" for User and Group subjects.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the object being referenced.
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace of the referenced object.  If the object kind is non-namespace, such as "User" or "Group", and this value is not empty
    /// the Authorizer should report an error.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for ClusterRole {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "ClusterRole";
    const NAME: &'static str = "clusterroles";
}
impl crate::HasMetadata for ClusterRole {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}


impl crate::Resource for ClusterRoleBinding {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "ClusterRoleBinding";
    const NAME: &'static str = "clusterrolebindings";
}
impl crate::HasMetadata for ClusterRoleBinding {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}


impl crate::Resource for Role {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "Role";
    const NAME: &'static str = "roles";
}
impl crate::HasMetadata for Role {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}


impl crate::Resource for RoleBinding {
    const API_VERSION: &'static str = "rbac.authorization.k8s.io/v1";
    const GROUP: &'static str = "rbac.authorization.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "RoleBinding";
    const NAME: &'static str = "rolebindings";
}
impl crate::HasMetadata for RoleBinding {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}

