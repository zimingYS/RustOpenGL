#[macro_use]
extern crate glium;

use std::fmt::Debug;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);



fn main() {
    //初始化设置
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().with_title("RuOpGL").build(&event_loop);

    //绘制三角形坐标
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    //顶点缓冲区
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //顶点着色器
    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    //片段着色器
    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    //绑定到Glium
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    //绘制窗口
    let mut target = display.draw();

    //绘制背景(R,G,B,不透明度)
    target.clear_color(0.0, 1.0, 1.0, 1.0);

    //绘制三角形图像
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();

    //绘制完整，将图片渲染到窗口
    target.finish().unwrap();

    //在事件CloseRequested发生之前使窗口不退出
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
