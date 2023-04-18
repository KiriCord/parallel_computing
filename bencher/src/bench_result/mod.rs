mod common;
pub mod multi_threaded;
pub mod single_threaded;
pub mod stats;
pub use multi_threaded::BenchResultMultiThreaded;
pub use single_threaded::BenchResultSingleThreaded;
