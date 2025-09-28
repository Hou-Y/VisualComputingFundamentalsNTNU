// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod scene_graph;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

mod mesh;


// Get a null pointer (equivalent to an offset of 0)
// ptr::null()


// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>, normals_o: Option<&Vec<f32>>) -> u32 {

    let mut vao = 0;
    let mut vbo = 0;
    let mut vbo_colors = 0;
    let mut vbo_normals = 0;
    let mut ebo = 0;

    //if normals present unwrap, otherwise 0 (empty)
    let mut dummy_vector: Vec<f32> = Vec::new();
    let normals = normals_o.unwrap_or(&dummy_vector);
    // VAO
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);

    //Note that when you bind a VAO with gl::BindVertexArray, the previous one is automatically unbound

    // VBO
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        vertices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    

    // EBO (index buffer)
    gl::GenBuffers(1, &mut ebo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        indices.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );

    // Configure vertex attribute (location = 0 in shader)
    gl::VertexAttribPointer(
        0,                // attribute index = location 0 in shader
        3,                // vec3
        gl::FLOAT,
        gl::FALSE,
        (3 * size_of::<f32>()) as i32, // stride (3 floats per vertex)
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0); //for VBO

    //VBO colour
        gl::GenBuffers(1, &mut vbo_colors);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_colors);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            byte_size_of_array(colors),
            colors.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            1, // location = 1 in shader
            4, // vec4 RGBA
            gl::FLOAT,
            gl::FALSE,
            (4 * size_of::<f32>()) as i32,
            ptr::null(),
        );
    gl::EnableVertexAttribArray(1); // for VBO colour
    //gl::BindVertexArray(0);

    //VBO for normals
    gl::GenBuffers(1, &mut vbo_normals);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo_normals);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        normals.as_ptr() as *const _,
        gl::STATIC_DRAW,
    );
    // Enable attribute location 1 for normals
    gl::VertexAttribPointer(
        2, //location 2 in shader
        3,
        gl::FLOAT,
        gl::FALSE,
        (3 * size_of::<f32>()) as i32,
        //(1 * size_of::<f32>()) as i32, // stride 1 float per vertex
        //provide a single normal vector for each vertex in your model
        ptr::null(),
    );
    gl::EnableVertexAttribArray(2);

    vao
}
//function to draw the scene
unsafe fn draw_scene( node: &scene_graph::SceneNode, shader: &shader::Shader, view_projection_matrix: &glm::Mat4, transform_so_far: &glm::Mat4,
     ){
    //transformations moved here
        /*let translation: glm::Mat4 = glm::translation(&node.position);
        let rotation_x = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let rotation_y = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let rotation_z = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let scaling: glm::Mat4 = glm::scaling(&node.scale);*/
        //let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        //let projection: glm::Mat4 = glm::perspective(std::f32::consts::FRAC_PI_4, window_aspect_ratio, 0.1, 1000.0);

        let translation = glm::translation(&node.position);
        let rotation_x  = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let rotation_y  = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let rotation_z  = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let scaling     = glm::scaling(&node.scale);

        let local_transform = translation * rotation_z * rotation_y * rotation_x * scaling * glm::translation(&-node.reference_point);

        let accumulated = transform_so_far * local_transform;

        // If this node is drawable (has VAO + indices), draw it
    if node.vao_id > 0 && node.index_count > 0 {
        shader.activate();

        // Send transform = VP * model
        let transform_loc = shader.get_uniform_location("view_projection");

        gl::BindVertexArray(node.vao_id);
        gl::UniformMatrix4fv(
            transform_loc,
            1,
            gl::FALSE,
            (view_projection_matrix*accumulated).as_ptr(),
        );
       
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
    }

    // Recurse into children
    for &child in &node.children {
        let child_type: &scene_graph::SceneNode = unsafe { &*child };
        draw_scene(child_type, shader, view_projection_matrix, &accumulated);
    }
}



fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
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

            //colour 
            /*The alpha channel denotes the transparency of the color. In this case, a value of 1 represents an entirely opaque color,
            while a value of 0 means the color is completely transparent.
            In Gloom-rs, transparency has been turned on by default. This feature must be explicitly enabled in OpenGL projects,*/
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            /*The gl::Enable() function enables a particular OpenGL feature. In this case, the parameter gl::BLEND specifies that you'd like to turn on alpha blending.
              The gl::BlendFunc() specifies how a transparent color should be mixed with colors from objects behind it.*/

            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here
        
        //fn mesh::Terrain::load("resources/lunarsurface.obj") -> Mesh;  //mesh 
        let terrain = mesh::Terrain::load("resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("resources/helicopter.obj");
        let terrain_vao = unsafe {
            create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, Some(&terrain.normals))
        };
        println!("length normal {}", terrain.normals.len()); //debug

        //// Define triangle vertices (3 points) and indices + KNOW YOUR DEPTH! :  Positive Z is deeper, negative is closer.
        // the triangle furthest away from the screen is drawn first e quelli piu'' vicini DOPO!
        let vertices: Vec<f32> = vec![
            // task2 q1
            /*0.6, -0.8, -1.2,
            0.0, 0.4, 0.0,
            -0.8, -0.2, 1.2*/

           /*
            // Triangle 1  //-> random visible triangles
            -0.8, -0.5, 0.0,  // left
            -0.4, -0.5, 0.0,  // right
            -0.6,  0.0, 0.0,  // top

            // Triangle 2
            0.0, -0.5, 0.0,  // left
            0.4, -0.5, 0.0,  // right
            0.2,  0.0, 0.0,  // top

            // Triangle 3
            -0.2,  0.2, 0.0,  // left
            0.2,  0.2, 0.0,  // right
            0.0,  0.6, 0.0,  // top
            */

            /*
            // Triangle 4
            0.6,  0.2, 0.0,  // left
            1.0,  0.2, 0.0,  // right
            0.8,  0.6, 0.0,  // top

            // Triangle 5
            -0.6,  0.3, 0.0,  // left
            -0.2,  0.3, 0.0,  // right
            -0.4,  0.7, 0.0,  // top*/

            /* 2 triangles forming a square
            -0.5, -0.5, 0.0,  // v0 bottom-left
            0.5, -0.5, 0.0,  // v1 bottom-right
            0.5,  0.5, 0.0,  // v2 top-right
            -0.5,  0.5, 0.0,  // v3 top-left*/

            /*Three partially overlapping triangles*/
            // Triangle 1 FARTHEST!
            -0.7, -0.5, 0.7,  // v0 bottom-left
            0.7, -0.5, 0.7,  // v1 bottom-right
            0.0,  0.5, 0.7,  // v2 top

            //
            -0.7,0.0,0.0,
            0.7,0.0,0.0,
            0.0,1.0,0.0,

            //
            -0.4,-0.2,-0.5,
            0.4,-0.2,-0.5,
            0.0,0.3,-0.5
        ];
        // Assignment2 Task3: Render the scene from Task 1b (three triangles with different colour per vertex)


        let indices: Vec<u32> = vec![

            /*
            0, 1, 2,    // Triangle 1
            3, 4, 5,    // Triangle 2
            6, 7, 8,    // Triangle 3
            9, 11, 10,  // Triangle 4
            12, 13, 14, // Triangle 5*/

            0,1,2,
            3,4,5,
            6,7,8
        ];


        //for every vertex 4 values [R,G,B, ALPHA/transparency]
        let colors: Vec<f32> = vec![

            //triangle 1 red
            1.0, 0.0, 0.0, 0.4, //v0
            1.0, 0.0, 0.0, 0.4,  
            1.0, 0.0, 0.0, 0.4,  


            //2 blue
            0.0, 0.0, 1.0, 0.5,
            0.0, 0.0, 1.0, 0.5,
            0.0, 0.0, 1.0, 0.5,

            //3 (green small triangle)
            0.0, 1.0, 0.0, 0.7, 
            0.0, 1.0, 0.0, 0.7,  
            0.0, 1.0, 0.0, 0.7

        ];

        //one normal per plane (triangle)
        let normals: Vec<f32> = vec![
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0
        ];

        //let my_vao = unsafe { create_vao(&vertices, &indices, &colors, Some(&normals)) };
        let body_vao = unsafe {
            create_vao(&helicopter.body.vertices,
                       &helicopter.body.indices,
                       &helicopter.body.colors,
                       Some(&helicopter.body.normals))
        };

        let door_vao = unsafe {
            create_vao(&helicopter.door.vertices,
                       &helicopter.door.indices,
                       &helicopter.door.colors,
                       Some(&helicopter.door.normals))
        };

        let main_rotor_vao = unsafe {
            create_vao(&helicopter.main_rotor.vertices,
                       &helicopter.main_rotor.indices,
                       &helicopter.main_rotor.colors,
                       Some(&helicopter.main_rotor.normals))
        };

        let tail_rotor_vao = unsafe {
            create_vao(&helicopter.tail_rotor.vertices,
                       &helicopter.tail_rotor.indices,
                       &helicopter.tail_rotor.colors,
                       Some(&helicopter.tail_rotor.normals))
        };

        // Terrain node
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain.index_count);

        // Helicopter nodes
        let mut helicopter_root_node = SceneNode::new();
        let mut body_node = SceneNode::from_vao(body_vao, helicopter.body.index_count);
        let mut door_node = SceneNode::from_vao(door_vao, helicopter.door.index_count);
        let mut main_rotor_node = SceneNode::from_vao(main_rotor_vao, helicopter.main_rotor.index_count);
        let mut tail_rotor_node = SceneNode::from_vao(tail_rotor_vao, helicopter.tail_rotor.index_count);


        // Hierarchy: body: door, main rotor, tail rotor
        body_node.add_child(&door_node);
        body_node.add_child(&main_rotor_node);
        body_node.add_child(&tail_rotor_node);

        // Helicopter root to body
        helicopter_root_node.add_child(&body_node);

        // Scene root : terrain + helicopter
        let mut scene_root = SceneNode::new();
        scene_root.add_child(&terrain_node);
        scene_root.add_child(&helicopter_root_node);

        //helicopter_root_node.print();
        //scene_root.print();
        //body_node.print();
        println!("Body indices: {}", helicopter.body.index_count);
        println!("Door indices: {}", helicopter.door.index_count);
        println!("Main rotor indices: {}", helicopter.main_rotor.index_count);
        println!("Tail rotor indices: {}", helicopter.tail_rotor.index_count);


        // == // Set up your shaders here
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };
        unsafe{simple_shader.activate()}; //moved activation outside the loop
        let transform_loc = unsafe { simple_shader.get_uniform_location("transform") };
        println!("LOCATION OF transform uniform: {}", transform_loc); //transform found!

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        /*
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./path/to/simple/shader.file")
                .link()
        };
        */


        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove

        //camera
                let mut cam_pos = glm::vec3(0.0, 0.0, 3.0);
                let mut cam_yaw: f32 = 0.0;
                let mut cam_pitch: f32 = 0.0;
                let cam_speed: f32 = 5.0;
                let rot_speed: f32 = 1.5;

        /*movimento verso un singolo asse (traslazione) l'oggetto va dalla parte opposta*/

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }

            // Handle keyboard input

            //direction names and sign in the code are wonky right now, sorry for that
            //just focus on the movement of the camera
            /* W goes nearer to the triangles (negative Z)
               S goes further away from the triangles (positive Z)
               A goes left (negative X)
               D goes right (positive X)
               Space goes up (positive Y)
               LShift goes down (negative Y)

               //rotation with arrows, all follows the recommended keybinds
            */

             //it is far easier to describe how your forearm moves compared to your arm than how it moves relative to the floor 
             //-> each node describes how its contents move relative to its PARENT node!! 
             //parent node must be evaluated before the child node

             /*order:  model matrix= parent matrix * own matrix
             Ready to go <- projection *view * model matrix
             Done with matrix stack tecnique to be called recursively by exploiting the existing program stack
                    Depth first order of visit
                    
             Reference point -> coordinate RELATIVE a una stessa origine che non necessariamente e'' l'origine degli assi della scena
             To rotate around the reference point: 
             first to move the obj such a way that the reference point lies at the origin
             second, apply the rotation
                -> per muovere un oggetto nell'origine basta traslarlo di un valore negativo rispetto alle coordinate di riferimento'
             third, move it back to where it was
             */

            if let Ok(keys) = pressed_keys.lock() {

                //walking simulator
                /*let forward = glm::vec3(cam_yaw.sin(), 0.0, cam_yaw.cos());
                let right   = glm::vec3(-forward.z, 0.0, forward.x);*/

                let rot_yaw: glm::Mat4 = glm::rotation(-cam_yaw, &glm::vec3(0.0, 1.0, 0.0));
                let rot_pitch: glm::Mat4 = glm::rotation(-cam_pitch, &glm::vec3(1.0, 0.0, 0.0));
                let forward = &glm::mat4_to_mat3(&(rot_pitch * rot_yaw));


                for key in keys.iter() {
                    match key {
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        //transposing
                        VirtualKeyCode::W => cam_pos += forward.transpose() * (cam_speed* &glm::vec3(0.0, 0.0, -1.0)) * delta_time * 10.0,
                        VirtualKeyCode::S => cam_pos += forward.transpose() * (cam_speed* &glm::vec3(0.0, 0.0, 1.0)) * delta_time * 10.0,
                        VirtualKeyCode::A => cam_pos += forward.transpose()  * (cam_speed* &glm::vec3(-1.0, 0.0, 0.0)) * delta_time * 10.0,
                        VirtualKeyCode::D => cam_pos += forward.transpose()  * (cam_speed* &glm::vec3(1.0, 0.0, 0.0)) * delta_time * 10.0,
                        //EDIT: added speed multiplier only for WASD to make it faster than up/down


                        /*// Movement old version 2
                        VirtualKeyCode::W => cam_pos += forward * cam_speed * delta_time,
                        VirtualKeyCode::S => cam_pos -= forward * cam_speed * delta_time,
                        VirtualKeyCode::A => cam_pos -= right   * cam_speed * delta_time,
                        VirtualKeyCode::D => cam_pos += right   * cam_speed * delta_time,*/

                        /*  //old only works when facing the front
                        VirtualKeyCode::D => cam_pos += forward * cam_speed * delta_time,
                        VirtualKeyCode::A => cam_pos -= forward * cam_speed * delta_time,
                        VirtualKeyCode::S => cam_pos -= right * cam_speed * delta_time,
                        VirtualKeyCode::W => cam_pos += right * cam_speed * delta_time,*/
                        //WASD works good when facing the front (XY) BUT becomes wonky once rotated right or left
                        // with rotated POV it's as if the WS and AD are swapped (opposite direction compared to regular/POV one)


                        //follow the poin of view of character in a 3D Space movement relative to the world
                        //WORK 
                        VirtualKeyCode::Space => cam_pos.y += cam_speed * delta_time,
                        VirtualKeyCode::LShift => cam_pos.y -= cam_speed * delta_time,

                        // Rotation (tripod style: yaw + pitch)
                        VirtualKeyCode::Right  => cam_yaw   -= rot_speed * delta_time,
                        VirtualKeyCode::Left => cam_yaw   += rot_speed * delta_time,
                        VirtualKeyCode::Up    => cam_pitch += rot_speed * delta_time,
                        VirtualKeyCode::Down  => cam_pitch -= rot_speed * delta_time,


                        // default handler:
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)



            unsafe {

                //let angle =  glm::radians(0.0);
                //value inside is in degrees, glm::radians converts it to radians
                
                
                let translation: glm::Mat4 = glm::translation(&glm::vec3(0.0, 0.0, -10.0));
                //let rotation: glm::Mat4 = glm::rotation(angle, &glm::vec3(1.0, 0.0, 0.0));
                //The angle of rotation should be in radians. You can convert degrees to radians using glm::radians()
                let scaling: glm::Mat4 = glm::scaling(&glm::vec3(1.0, 1.0, 1.0));

                let fovy = std::f32::consts::FRAC_PI_4; // 45 degrees in radians
                let aspect = window_aspect_ratio;        // width / height
                let near = 0.1;
                let far = 1000.0;  //far increased to 1000

                let projection: glm::Mat4 = glm::perspective(aspect, fovy, near, far);
                //You see, the projection matrix returned by glm::perspective() flips the z-axis.
                //so right now you are seeing this image from the back BUT right now the red triangle is the FIRST that gets drawn
                //and it is also the CLOSEST to us (in other word the other triangles in the back are not visible because it is discarded)

                //The second parameter (fovy) specifies the vertical Field Of View (FOV) the camera is capable of capturing.
                //That is, the angle //between the top and the bottom plane of the view frustum.

                // build view matrix (tripod camera)
                let rot_yaw: glm::Mat4 = glm::rotation(-cam_yaw, &glm::vec3(0.0, 1.0, 0.0));
                let rot_pitch: glm::Mat4 = glm::rotation(-cam_pitch, &glm::vec3(1.0, 0.0, 0.0));
                let trans = glm::translation(&-cam_pos);
                let view = rot_pitch * rot_yaw * trans;
                //let view = rot_pitch * rot_yaw * translation;

                let transform: glm::Mat4 = projection * view;
                let identity: glm::Mat4 = glm::identity();
                //apply the perspective projection on the transformation matrix sent into the vertex shader
                //FIRST TRANSFORMATIONS THEN SHADERS

                /*our  camera matrix needs to translate the world in such a way that the camera's position becomes the
                origin, and rotate it such that the world's x, y and z axes becomes aligned with the camera 's orientation.*/

                //implement rotation up/down + rotation left/right + 3D movement 
                /*Avoid using glm::look_at
                build entire transformation from scratch each frame*/


                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                //gl::ClearColor(1.0, 0.046, 0.008, 1.0); // very reddish
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                // == // Issue the necessary gl:: commands to draw your scene here
                gl::UniformMatrix4fv(
                    transform_loc,           // location
                    1,                       // count
                    gl::FALSE,               // no need to transpose (glm is already column-major)
                    transform.as_ptr(),      // pointer to first element
                );

                /*gl::BindVertexArray(my_vao);*/

                draw_scene(&scene_root, &simple_shader, &transform, &identity);
             

            /* OLD 
                gl::UniformMatrix4fv(
                    transform_loc,           // location
                    1,                       // count
                    gl::FALSE,               // no need to transpose (glm is already column-major)
                    transform.as_ptr(),      // pointer to first element
                );

                gl::BindVertexArray(my_vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                gl::BindVertexArray(terrain_vao);

                gl::DrawElements(
                    gl::TRIANGLES,
                    terrain.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
                //make these normals a part of our VAO just like with color

                // Draw helicopter body
                gl::BindVertexArray(body_vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.body.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                // Draw door
                gl::BindVertexArray(door_vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.door.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                // Draw main rotor
                gl::BindVertexArray(main_rotor_vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.main_rotor.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

                // Draw tail rotor
                gl::BindVertexArray(tail_rotor_vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    helicopter.tail_rotor.index_count,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );

            END OLD*/

            /*Any time you use any of the functions from the glm library, you should be specifying the type for the resulting variable as glm::Mat4*/
            }


            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


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
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
