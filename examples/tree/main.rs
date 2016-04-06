#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate cgmath;
extern crate itertools;

use cgmath::{ Vector3, Vector4, Point3, Matrix4, SquareMatrix };

type Pnt = Vector4<f32>;
type Mat = Matrix4<f32>;

mod trunk {
    use itertools::Itertools;
    use cgmath::{ Quaternion, Rotation3, Matrix4, SquareMatrix, vec3, rad };
    use { Pnt, Mat, Vertex };

    fn make_points(n: usize, t: &Mat) -> Vec<Pnt> {
        use std::f32::consts::PI;
        (0..(n + 1)).map(|i| (i as f32) * 2.0 * PI / (n as f32))
            .map(|a| t * Pnt::new(a.cos(), a.sin(), 0.0, 1.0))
            .collect()
    }

    fn make_segment(base: &Mat, head: &Mat) -> Vec<Pnt> {
        let trunk_faces = 6;
        let base_v = make_points(trunk_faces, base);
        let head_v = make_points(trunk_faces, head);
        head_v.into_iter().interleave(base_v).collect()
    }

    pub fn make_trunk() -> Vec<Vec<Vertex>> {
        let segments = 5;
        let base = Matrix4::identity();
        let head =
            Matrix4::from(Quaternion::from_angle_z(rad(0.1))) *
            Matrix4::from_translation(vec3(0.0, 0.0, 1.0));

        vec![ make_segment(&base, &head)
              .into_iter().map(|p| Vertex { pos: p.into() }).collect() ]
    }
}

gfx_vertex_struct! {
    Vertex {
        pos: [f32; 4] = "vertex_pos",
    }
}

gfx_pipeline! {
    pipeline {
        vertexbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "transform",
        out_color: gfx::RenderTarget<::gfx_app::ColorFormat> = "out_color",
        out_depth: gfx::DepthTarget<::gfx_app::DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

struct App<R: gfx::Resources> {
    state: gfx::PipelineState<R, pipeline::Meta>,
    transform: Matrix4<f32>,
    out_color: gfx::handle::RenderTargetView<R, gfx_app::ColorFormat>,
    out_depth: gfx::handle::DepthStencilView<R, gfx_app::DepthFormat>,

    tree_segments: Vec<(pipeline::Data<R>, gfx::Slice<R>)>,
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

        let shaderset = factory.create_shader_set(
            vs.select(init.backend).unwrap(),
            ps.select(init.backend).unwrap()).unwrap();

        let state = factory.create_pipeline_state(
            &shaderset,
            gfx::Primitive::TriangleStrip,
            gfx::state::Rasterizer::new_fill(gfx::state::CullFace::Nothing),
            pipeline::new()).unwrap();

        let transform =
            cgmath::perspective(cgmath::deg(45.0), init.aspect_ratio, 1.0, 10.0) *
            Matrix4::look_at(
                Point3::new(1.5, -5.0, 3.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::unit_z());

        let segments = trunk::make_trunk().into_iter().map(|vertices| {
            let (vbuf, slice) = factory.create_vertex_buffer(&vertices);
            let data = pipeline::Data {
                vertexbuf: vbuf,
                transform: transform.into(),
                out_color: init.color.clone(),
                out_depth: init.depth.clone(),
            };
            (data, slice)
        }).collect();

        App {
            state: state,
            transform: transform,
            out_color: init.color,
            out_depth: init.depth,
            tree_segments: segments,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        encoder.clear(&self.out_color, [ 0.6, 0.6, 0.9, 1.0 ]);
        for &(ref data, ref slice) in self.tree_segments.iter() {
            encoder.draw(slice, &self.state, data);
        }
        encoder.clear_depth(&self.out_depth, 1.0);
    }
}

fn main() {
    use gfx_app::Application;
    App::launch_default("Tree example");
}
