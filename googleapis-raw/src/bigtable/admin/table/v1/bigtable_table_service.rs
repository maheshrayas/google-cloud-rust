// This file is generated by rust-protobuf 2.23.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/bigtable/admin/table/v1/bigtable_table_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n;google/bigtable/admin/table/v1/bigtable_table_service.proto\x12\x1ego\
    ogle.bigtable.admin.table.v1\x1a\x1cgoogle/api/annotations.proto\x1a8goo\
    gle/bigtable/admin/table/v1/bigtable_table_data.proto\x1aDgoogle/bigtabl\
    e/admin/table/v1/bigtable_table_service_messages.proto\x1a\x1bgoogle/pro\
    tobuf/empty.proto2\xbe\x0c\n\x14BigtableTableService\x12\xa4\x01\n\x0bCr\
    eateTable\x122.google.bigtable.admin.table.v1.CreateTableRequest\x1a%.go\
    ogle.bigtable.admin.table.v1.Table\":\x82\xd3\xe4\x93\x024\"//v1/{name=p\
    rojects/*/zones/*/clusters/*}/tables:\x01*\x12\xac\x01\n\nListTables\x12\
    1.google.bigtable.admin.table.v1.ListTablesRequest\x1a2.google.bigtable.\
    admin.table.v1.ListTablesResponse\"7\x82\xd3\xe4\x93\x021\x12//v1/{name=\
    projects/*/zones/*/clusters/*}/tables\x12\x9d\x01\n\x08GetTable\x12/.goo\
    gle.bigtable.admin.table.v1.GetTableRequest\x1a%.google.bigtable.admin.t\
    able.v1.Table\"9\x82\xd3\xe4\x93\x023\x121/v1/{name=projects/*/zones/*/c\
    lusters/*/tables/*}\x12\x94\x01\n\x0bDeleteTable\x122.google.bigtable.ad\
    min.table.v1.DeleteTableRequest\x1a\x16.google.protobuf.Empty\"9\x82\xd3\
    \xe4\x93\x023*1/v1/{name=projects/*/zones/*/clusters/*/tables/*}\x12\x9e\
    \x01\n\x0bRenameTable\x122.google.bigtable.admin.table.v1.RenameTableReq\
    uest\x1a\x16.google.protobuf.Empty\"C\x82\xd3\xe4\x93\x02=\"8/v1/{name=p\
    rojects/*/zones/*/clusters/*/tables/*}:rename:\x01*\x12\xca\x01\n\x12Cre\
    ateColumnFamily\x129.google.bigtable.admin.table.v1.CreateColumnFamilyRe\
    quest\x1a,.google.bigtable.admin.table.v1.ColumnFamily\"K\x82\xd3\xe4\
    \x93\x02E\"@/v1/{name=projects/*/zones/*/clusters/*/tables/*}/columnFami\
    lies:\x01*\x12\xbf\x01\n\x12UpdateColumnFamily\x12,.google.bigtable.admi\
    n.table.v1.ColumnFamily\x1a,.google.bigtable.admin.table.v1.ColumnFamily\
    \"M\x82\xd3\xe4\x93\x02G\x1aB/v1/{name=projects/*/zones/*/clusters/*/tab\
    les/*/columnFamilies/*}:\x01*\x12\xb3\x01\n\x12DeleteColumnFamily\x129.g\
    oogle.bigtable.admin.table.v1.DeleteColumnFamilyRequest\x1a\x16.google.p\
    rotobuf.Empty\"J\x82\xd3\xe4\x93\x02D*B/v1/{name=projects/*/zones/*/clus\
    ters/*/tables/*/columnFamilies/*}\x12\xb2\x01\n\x0eBulkDeleteRows\x125.g\
    oogle.bigtable.admin.table.v1.BulkDeleteRowsRequest\x1a\x16.google.proto\
    buf.Empty\"Q\x82\xd3\xe4\x93\x02K\"F/v1/{table_name=projects/*/zones/*/c\
    lusters/*/tables/*}:bulkDeleteRows:\x01*B\x87\x01\n\"com.google.bigtable\
    .admin.table.v1B\x1aBigtableTableServicesProtoP\x01ZCgoogle.golang.org/g\
    enproto/googleapis/bigtable/admin/table/v1;tableJ\x8c\x13\n\x06\x12\x04\
    \x0e\0g\x01\n\xbd\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\
    \x202017\x20Google\x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20\
    License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20n\
    ot\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\
    \x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20Lice\
    nse\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-\
    2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\0'\n\t\
    \n\x02\x03\0\x12\x03\x12\0&\n\t\n\x02\x03\x01\x12\x03\x13\0B\n\t\n\x02\
    \x03\x02\x12\x03\x14\0N\n\t\n\x02\x03\x03\x12\x03\x15\0%\n\x08\n\x01\x08\
    \x12\x03\x17\0Z\n\t\n\x02\x08\x0b\x12\x03\x17\0Z\n\x08\n\x01\x08\x12\x03\
    \x18\0\"\n\t\n\x02\x08\n\x12\x03\x18\0\"\n\x08\n\x01\x08\x12\x03\x19\0;\
    \n\t\n\x02\x08\x08\x12\x03\x19\0;\n\x08\n\x01\x08\x12\x03\x1a\0;\n\t\n\
    \x02\x08\x01\x12\x03\x1a\0;\n\xa9\x01\n\x02\x06\0\x12\x04\x1f\0g\x01\x1a\
    \x9c\x01\x20Service\x20for\x20creating,\x20configuring,\x20and\x20deleti\
    ng\x20Cloud\x20Bigtable\x20tables.\n\x20Provides\x20access\x20to\x20the\
    \x20table\x20schemas\x20only,\x20not\x20the\x20data\x20stored\x20within\
    \x20the\n\x20tables.\n\n\n\n\x03\x06\0\x01\x12\x03\x1f\x08\x1c\n\xad\x01\
    \n\x04\x06\0\x02\0\x12\x04#\x02(\x03\x1a\x9e\x01\x20Creates\x20a\x20new\
    \x20table,\x20to\x20be\x20served\x20from\x20a\x20specified\x20cluster.\n\
    \x20The\x20table\x20can\x20be\x20created\x20with\x20a\x20full\x20set\x20\
    of\x20initial\x20column\x20families,\n\x20specified\x20in\x20the\x20requ\
    est.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03#\x06\x11\n\x0c\n\x05\x06\0\
    \x02\0\x02\x12\x03#\x12$\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03#/4\n\r\n\
    \x05\x06\0\x02\0\x04\x12\x04$\x04'\x06\n\x11\n\t\x06\0\x02\0\x04\xb0\xca\
    \xbc\"\x12\x04$\x04'\x06\nN\n\x04\x06\0\x02\x01\x12\x04+\x02/\x03\x1a@\
    \x20Lists\x20the\x20names\x20of\x20all\x20tables\x20served\x20from\x20a\
    \x20specified\x20cluster.\n\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03+\x06\
    \x10\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03+\x11\"\n\x0c\n\x05\x06\0\x02\
    \x01\x03\x12\x03+-?\n\r\n\x05\x06\0\x02\x01\x04\x12\x04,\x04.\x06\n\x11\
    \n\t\x06\0\x02\x01\x04\xb0\xca\xbc\"\x12\x04,\x04.\x06\nV\n\x04\x06\0\
    \x02\x02\x12\x042\x026\x03\x1aH\x20Gets\x20the\x20schema\x20of\x20the\
    \x20specified\x20table,\x20including\x20its\x20column\x20families.\n\n\
    \x0c\n\x05\x06\0\x02\x02\x01\x12\x032\x06\x0e\n\x0c\n\x05\x06\0\x02\x02\
    \x02\x12\x032\x0f\x1e\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x032).\n\r\n\x05\
    \x06\0\x02\x02\x04\x12\x043\x045\x06\n\x11\n\t\x06\0\x02\x02\x04\xb0\xca\
    \xbc\"\x12\x043\x045\x06\nJ\n\x04\x06\0\x02\x03\x12\x049\x02=\x03\x1a<\
    \x20Permanently\x20deletes\x20a\x20specified\x20table\x20and\x20all\x20o\
    f\x20its\x20data.\n\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x039\x06\x11\n\x0c\
    \n\x05\x06\0\x02\x03\x02\x12\x039\x12$\n\x0c\n\x05\x06\0\x02\x03\x03\x12\
    \x039/D\n\r\n\x05\x06\0\x02\x03\x04\x12\x04:\x04<\x06\n\x11\n\t\x06\0\
    \x02\x03\x04\xb0\xca\xbc\"\x12\x04:\x04<\x06\n{\n\x04\x06\0\x02\x04\x12\
    \x04A\x02F\x03\x1am\x20Changes\x20the\x20name\x20of\x20a\x20specified\
    \x20table.\n\x20Cannot\x20be\x20used\x20to\x20move\x20tables\x20between\
    \x20clusters,\x20zones,\x20or\x20projects.\n\n\x0c\n\x05\x06\0\x02\x04\
    \x01\x12\x03A\x06\x11\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03A\x12$\n\x0c\
    \n\x05\x06\0\x02\x04\x03\x12\x03A/D\n\r\n\x05\x06\0\x02\x04\x04\x12\x04B\
    \x04E\x06\n\x11\n\t\x06\0\x02\x04\x04\xb0\xca\xbc\"\x12\x04B\x04E\x06\nE\
    \n\x04\x06\0\x02\x05\x12\x04I\x02N\x03\x1a7\x20Creates\x20a\x20new\x20co\
    lumn\x20family\x20within\x20a\x20specified\x20table.\n\n\x0c\n\x05\x06\0\
    \x02\x05\x01\x12\x03I\x06\x18\n\x0c\n\x05\x06\0\x02\x05\x02\x12\x03I\x19\
    2\n\x0c\n\x05\x06\0\x02\x05\x03\x12\x03I=I\n\r\n\x05\x06\0\x02\x05\x04\
    \x12\x04J\x04M\x06\n\x11\n\t\x06\0\x02\x05\x04\xb0\xca\xbc\"\x12\x04J\
    \x04M\x06\nG\n\x04\x06\0\x02\x06\x12\x04Q\x02V\x03\x1a9\x20Changes\x20th\
    e\x20configuration\x20of\x20a\x20specified\x20column\x20family.\n\n\x0c\
    \n\x05\x06\0\x02\x06\x01\x12\x03Q\x06\x18\n\x0c\n\x05\x06\0\x02\x06\x02\
    \x12\x03Q\x19%\n\x0c\n\x05\x06\0\x02\x06\x03\x12\x03Q0<\n\r\n\x05\x06\0\
    \x02\x06\x04\x12\x04R\x04U\x06\n\x11\n\t\x06\0\x02\x06\x04\xb0\xca\xbc\"\
    \x12\x04R\x04U\x06\nR\n\x04\x06\0\x02\x07\x12\x04Y\x02^\x03\x1aD\x20Perm\
    anently\x20deletes\x20a\x20specified\x20column\x20family\x20and\x20all\
    \x20of\x20its\x20data.\n\n\x0c\n\x05\x06\0\x02\x07\x01\x12\x03Y\x06\x18\
    \n\x0c\n\x05\x06\0\x02\x07\x02\x12\x03Y\x192\n\x0c\n\x05\x06\0\x02\x07\
    \x03\x12\x03Z\x0f$\n\r\n\x05\x06\0\x02\x07\x04\x12\x04[\x04]\x06\n\x11\n\
    \t\x06\0\x02\x07\x04\xb0\xca\xbc\"\x12\x04[\x04]\x06\nO\n\x04\x06\0\x02\
    \x08\x12\x04a\x02f\x03\x1aA\x20Delete\x20all\x20rows\x20in\x20a\x20table\
    \x20corresponding\x20to\x20a\x20particular\x20prefix\n\n\x0c\n\x05\x06\0\
    \x02\x08\x01\x12\x03a\x06\x14\n\x0c\n\x05\x06\0\x02\x08\x02\x12\x03a\x15\
    *\n\x0c\n\x05\x06\0\x02\x08\x03\x12\x03a5J\n\r\n\x05\x06\0\x02\x08\x04\
    \x12\x04b\x04e\x06\n\x11\n\t\x06\0\x02\x08\x04\xb0\xca\xbc\"\x12\x04b\
    \x04e\x06b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
