#[cfg(feature = "file")]
mod file {
    include!(concat!(env!("OUT_DIR"), "/File.rs"));
}
#[cfg(feature = "file")]
pub use file::org::apache::arrow::flatbuf as File;

#[cfg(feature = "message")]
mod message {
    include!(concat!(env!("OUT_DIR"), "/Message.rs"));
}
#[cfg(feature = "message")]
pub use message::org::apache::arrow::flatbuf as Message;

#[cfg(feature = "schema")]
#[allow(non_camel_case_types)]
mod schema {
    include!(concat!(env!("OUT_DIR"), "/Schema.rs"));
}
#[cfg(feature = "schema")]
pub use schema::org::apache::arrow::flatbuf as Schema;

#[cfg(feature = "sparse_tensor")]
mod sparsetensor {
    include!(concat!(env!("OUT_DIR"), "/SparseTensor.rs"));
}
#[cfg(feature = "sparse_tensor")]
pub use sparsetensor::org::apache::arrow::flatbuf as SparseTensor;

#[cfg(feature = "tensor")]
mod tensor {
    include!(concat!(env!("OUT_DIR"), "/Tensor.rs"));
}
#[cfg(feature = "tensor")]
pub use tensor::org::apache::arrow::flatbuf as Tensor;

#[cfg(feature = "plasma_common")]
mod _common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}
#[cfg(feature = "plasma_common")]
pub use _common::plasma::flatbuf as common;

#[cfg(feature = "plasma_")]
mod _plasma {
    include!(concat!(env!("OUT_DIR"), "/plasma.rs"));
}
#[cfg(feature = "plasma_")]
pub use _plasma::plasma::flatbuf as plasma;

#[cfg(feature = "feather")]
mod _feather {
    include!(concat!(env!("OUT_DIR"), "/feather.rs"));
}
#[cfg(feature = "feather")]
pub use _feather::arrow::ipc::feather::fbs as feather;

#[cfg(feature = "flight")]
pub mod flight {
    include!(concat!(env!("OUT_DIR"), "/arrow.flight.protocol.rs"));
}
