extern crate glutin;
extern crate gl;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use gl::types::*;
use std::ptr;

// Define the vertices of the triangle along with their color attributes (R, G, B).
const VERTICES: [f32; 15] = [
    -0.5, -0.5, 1.0, 0.0, 0.0, // Vertex 1: (x, y, R, G, B)
    0.5, -0.5, 0.0, 1.0, 0.0, // Vertex 2: (x, y, R, G, B)
    0.0, 0.5, 0.0, 0.0, 1.0,  // Vertex 3: (x, y, R, G, B)
];

fn main() {
    // Create an event loop for managing window events.
    let event_loop = EventLoop::new();

    // Create a window with a specified title.
    let window_builder = WindowBuilder::new().with_title("OpenGL9");

    // Build the windowed context for OpenGL rendering.
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap();

    // Make the OpenGL context current.
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // Load OpenGL function pointers.
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    // Create and bind a Vertex Array Object (VAO).
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
    }

    // Create and bind a Vertex Buffer Object (VBO).
    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTICES.len() * std::mem::size_of::<f32>()) as isize,
            VERTICES.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
    }

    // Set up vertex attributes for position and color.
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid);
    }

    // Start the event loop.
    event_loop.run(move |event, _, control_flow| {
        match event {
            // Handle window close event.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                // Set the clear color to white and clear the screen.
                unsafe {
                    gl::ClearColor(1.0, 1.0, 1.0, 1.0); // Set clear color to white (RGB: 1.0, 1.0, 1.0)
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                // Bind the VAO and draw the triangle.
                unsafe {
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }

                // Swap buffers to display the rendered content.
                windowed_context.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
}
