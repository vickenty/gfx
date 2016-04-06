#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate cgmath;

gfx_pipeline! {
    pipeline {
        out_color: gfx::RenderTarget<::gfx_app::ColorFormat> = "out_color",
        out_depth: gfx::DepthTarget<::gfx_app::DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

struct App<R: gfx::Resources> {
    state: gfx::PipelineState<R, pipeline::Meta>,
    out_color: gfx::handle::RenderTargetView<R, gfx_app::ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<R, gfx_app::DepthFormat>,
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(mut factory: F, init: gfx_app::Init<R>) -> Self {
        use gfx::traits::FactoryExt;
        
        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tree_150.glslv"),
            ..gfx_app::shade::Source::empty()
        };

        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shader/tree_150.glslf"),
            ..gfx_app::shade::Source::empty()
        };
        
        let state = factory.create_pipeline_simple(
            vs.select(init.backend).unwrap(),
            ps.select(init.backend).unwrap(),
            gfx::state::CullFace::Nothing,
            pipeline::new()).unwrap();
        
        App {
            state: state,
            out_color: init.color,
            out_depth: init.depth,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.out_color, [ 0.6, 0.6, 0.9, 1.0 ]);
        encoder.clear_depth(&self.out_depth, 1.0);
    }
}

fn main() {
    use gfx_app::Application;
    App::launch_default("Tree example");
}
