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
    //更新顶点位置
    let shape = vec![
        Vertex { position: [-0.5 , -0.5] },
        Vertex { position: [ 0.0 ,  0.5] },
        Vertex { position: [ 0.5 , -0.25] }
    ];
    //顶点缓冲区
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //顶点着色器
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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


    //t为移动量
    let mut t: f32 = 0.0;

    //在事件CloseRequested发生之前使窗口不退出
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                //接收到CloseRequested时退出循环
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                //当窗口大小改变时调整
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                //绘制图像
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // 把绘制代码放到这里！
                    t += 0.2;
                    let x = t.sin() * 0.5;

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [  x, 0.0, 0.0, 1.0f32],
                        ]
                    };
                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                },
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                _window.request_redraw();
            },
            _ => (),
        };
    }).unwrap();
}
