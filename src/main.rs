extern crate nalgebra_glm as glm;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

pub mod shader;
pub mod utils;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    fn flatten(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        vec.extend_from_slice(&self.position);
        vec.extend_from_slice(&self.color);
        vec
    }
}

struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    fn new(v_1: Vertex, v_2: Vertex, v_3: Vertex) -> Triangle {
        Triangle {
            vertices: [v_1, v_2, v_3],
        }
    }

    fn flatten(&self) -> Vec<f32> {
        let mut vec = Vec::new();
        for vertex in &self.vertices {
            vec.extend_from_slice(&vertex.flatten());
        }
        vec
    }
}

unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>) -> u32 {
    // * Generate a VAO and bind it
    let mut vao_ids: u32 = 0;
    gl::GenVertexArrays(1, &mut vao_ids as *mut u32);
    gl::BindVertexArray(vao_ids);

    // * Generate a VBO and bind it
    let mut vbo_ids: u32 = 0;
    gl::GenBuffers(1, &mut vbo_ids as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_ids);

    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        utils::byte_size_of_array(vertices),
        utils::pointer_to_array(vertices),
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        // For the position
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        utils::size_of::<f32>() * 7,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    gl::VertexAttribPointer(
        // For the color
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        utils::size_of::<f32>() * 7,
        utils::offset::<f32>(3),
    );
    gl::EnableVertexAttribArray(1);

    // * Generate a IBO and bind it
    let mut ibo_ids: u32 = 0;
    gl::GenBuffers(1, &mut ibo_ids as *mut u32);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_ids);

    // * Fill it with data
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        utils::byte_size_of_array(indices),
        utils::pointer_to_array(indices),
        gl::STATIC_DRAW,
    );

    // * Return the ID of the VAO
    vao_ids
}

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("studios-systems")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));

    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));

    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        }

        let white = [1.0, 1.0, 1.0, 1.0];

        let triangle_1 = Triangle::new(
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.0, -0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.0, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
        )
        .flatten();

        let triangle_2 = Triangle::new(
            Vertex {
                position: [-0.5, 0.0, 0.0],
                color: white,
            },
            Vertex {
                position: [0.0, -0.5, 0.0],
                color: white,
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                color: white,
            },
        )
        .flatten();

        let triangle_vec: Vec<f32> = vec![triangle_1, triangle_2].concat();
        let indices: Vec<u32> = vec![0, 1, 2, 3, 4, 5];

        let triangle_vao = unsafe { create_vao(&triangle_vec, &indices) };

        let shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };

        unsafe {
            gl::BindVertexArray(triangle_vao);
            shader.activate();
        }

        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;

        let mut cam_pos: glm::Vec3 = glm::vec3(0.0, 0.0, 5.0);
        let mut cam_dir: glm::Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let mut yaw: f32 = -90.0;
        let mut pitch: f32 = 0.0;

        let mut cam_front: glm::Vec3 = glm::vec3(0.0, 0.0, -1.0);
        let cam_up: glm::Vec3 = glm::vec3(0.0, 1.0, 0.0);

        let move_speed: f32 = 10.0;
        let cam_speed: f32 = 100.0;

        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let _elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Resized");
                    unsafe {
                        gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                    }
                }
            }

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html
                        VirtualKeyCode::D => {
                            //print
                            cam_pos += move_speed
                                * delta_time
                                * glm::normalize(&glm::cross(&cam_front, &cam_up));
                        }
                        VirtualKeyCode::A => {
                            cam_pos -= move_speed
                                * delta_time
                                * glm::normalize(&glm::cross(&cam_front, &cam_up));
                        }

                        VirtualKeyCode::Space => {
                            cam_pos += move_speed * delta_time * cam_up;
                        }
                        VirtualKeyCode::LShift => {
                            cam_pos -= move_speed * delta_time * cam_up;
                        }

                        VirtualKeyCode::W => {
                            cam_pos += move_speed * delta_time * cam_front;
                        }
                        VirtualKeyCode::S => {
                            cam_pos -= move_speed * delta_time * cam_front;
                        }

                        VirtualKeyCode::Up => {
                            pitch += delta_time * cam_speed;
                        }
                        VirtualKeyCode::Down => {
                            pitch -= delta_time * cam_speed;
                        }

                        VirtualKeyCode::Left => {
                            yaw -= delta_time * cam_speed;
                        }
                        VirtualKeyCode::Right => {
                            yaw += delta_time * cam_speed;
                        }

                        // default handler:
                        _ => {}
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                // == // Optionally access the acumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            unsafe {
                let mut transformation_matrix: glm::Mat4 = glm::identity();
                let mut view_matrix: glm::Mat4 = glm::identity();
                let model_matrix: glm::Mat4 = glm::identity();

                cam_dir.x = yaw.to_radians().cos() * pitch.to_radians().cos();
                cam_dir.y = pitch.to_radians().sin();
                cam_dir.z = yaw.to_radians().sin() * pitch.to_radians().cos();

                cam_front = glm::normalize(&cam_dir);

                let look_at: glm::Mat4 = glm::look_at(&cam_pos, &(cam_pos + cam_front), &cam_up);

                view_matrix = look_at * view_matrix;

                let projection_matrix: glm::Mat4 =
                    glm::perspective(window_aspect_ratio, glm::half_pi(), 1.0, 100.0);

                transformation_matrix =
                    projection_matrix * view_matrix * model_matrix * transformation_matrix;

                gl::UniformMatrix4fv(4, 1, gl::TRUE, transformation_matrix.as_ptr());

                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                gl::DrawElements(
                    gl::TRIANGLES,
                    (triangle_vec.len() / 7) as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size! width: {}, height: {}",
                    physical_size.width, physical_size.height
                );
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
