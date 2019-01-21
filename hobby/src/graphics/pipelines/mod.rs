macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize } as u32
    }
}

pub(crate) mod basic_pipeline;
pub(crate) mod mvp;
pub(crate) mod pipeline;
pub(crate) mod shader;

pub(crate) use self::basic_pipeline::BasicPipeline;
pub(crate) use self::basic_pipeline::BasicVertex;
pub(crate) use self::mvp::Mvp;
pub(crate) use self::mvp::MvpBuffers;
pub(crate) use self::pipeline::Pipeline;
