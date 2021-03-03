use vulkano::framebuffer::{PassDependencyDescription, PassDescription, StoreOp, LoadOp, AttachmentDescription, RenderPassDesc, RenderPassDescClearValues};
use vulkano::image::ImageLayout;

use vulkano::format::{ClearValue, Format};
use vulkano::SafeDeref;
use std::ops::Deref;
#[derive(Copy, Clone)]
pub struct EguiRenderPassDesc {
    pub color: (Format, u32),
}

unsafe impl RenderPassDescClearValues<Vec<ClearValue>> for EguiRenderPassDesc{
    fn convert_clear_values(&self, x: Vec<ClearValue>) -> Box<dyn Iterator<Item=ClearValue>> {
        unimplemented!()
    }
}
unsafe impl RenderPassDesc for EguiRenderPassDesc{
    fn num_attachments(&self) -> usize {
        1
    }

    fn attachment_desc(&self, num: usize) -> Option<AttachmentDescription> {
        if num==0{
            Some(AttachmentDescription{
                format: self.color.0,
                samples: 1,
                load: LoadOp::Clear,
                store: StoreOp::Store,
                stencil_load: LoadOp::Clear,
                stencil_store: StoreOp::Store,
                initial_layout: ImageLayout::ColorAttachmentOptimal,
                final_layout: ImageLayout::ColorAttachmentOptimal
            })}
        else{
            None
        }
    }

    fn num_subpasses(&self) -> usize {
        1
    }

    fn subpass_desc(&self, num: usize) -> Option<PassDescription> {
        unimplemented!()
    }

    fn num_dependencies(&self) -> usize {
        unimplemented!()
    }

    fn dependency_desc(&self, num: usize) -> Option<PassDependencyDescription> {
        unimplemented!()
    }
}