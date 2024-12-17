pub mod area;
pub use area::*;

pub struct AtlasMetadata<TS: PartialOrd, T: AtlasArea<TS>> {
    pub areas: Vec<T>
}

pub struct AtlasPacker<TS: PartialOrd, T: AtlasArea<TS>> {

}

impl<TS: PartialOrd, T: AtlasArea<TS>> AtlasPacker<TS, T> {
    
}