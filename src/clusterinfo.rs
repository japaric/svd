
use xmltree::Element;
use either::Either;


use elementext::ElementExt;

use helpers::{ParseElem, EncodeElem, new_element};
use parse;

use access::Access;
use register::Register;
use cluster::Cluster;
use registercluster::cluster_register_parse;


#[derive(Clone, Debug, PartialEq)]
pub struct ClusterInfo {
    pub name: String,
    pub description: String,
    pub header_struct_name: String,
    pub address_offset: u32,
    pub size: Option<u32>,
    pub access: Option<Access>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
    pub children: Vec<Either<Register, Cluster>>,
    // Reserve the right to add more fields to this struct
    pub(crate) _extensible: (),
}

impl ParseElem for ClusterInfo {
    fn parse(tree: &Element) -> ClusterInfo {
        ClusterInfo {
            name: try_get_child!(tree.get_child_text("name")),
            description: try_get_child!(tree.get_child_text("description")),
            header_struct_name: try_get_child!(tree.get_child_text("headerStructName")),
            address_offset: {
                try_get_child!(parse::u32(try_get_child!(tree.get_child("addressOffset"))))
            },
            size: tree.get_child("size").map(
                |t| try_get_child!(parse::u32(t)),
            ),
            access: tree.get_child("access").map(Access::parse),
            reset_value: tree.get_child("resetValue").map(|t| {
                try_get_child!(parse::u32(t))
            }),
            reset_mask: tree.get_child("resetMask").map(
                |t| try_get_child!(parse::u32(t)),
            ),
            children: tree.children
                .iter()
                .filter(|t| t.name == "register" || t.name == "cluster")
                .map(cluster_register_parse)
                .collect(),
            _extensible: (),
        }
    }
}

impl EncodeElem for ClusterInfo {
    fn encode(&self) -> Element {
        new_element("fake", None)
    }
}
