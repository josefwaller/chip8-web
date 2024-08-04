use crate::dom::get_canvas;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader};

const VERTEX_SHADER: &str = r##"#version 300 es
in vec4 position;
in vec3 color;
out vec3 gColor;

void main() {
    gl_Position = position;
    gColor = color;
}
"##;

const FRAGMENT_SHADER: &str = r##"#version 300 es

precision highp float;
out vec4 outColor;
in vec3 gColor;

void main() {
    outColor = vec4(gColor, 1);
}
"##;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct RenderInfo {
    pub context: WebGl2RenderingContext,
    pub colors_buf: WebGlBuffer,
    pub indices_len: usize,
}
pub fn init_wegl() -> RenderInfo {
    let context = get_canvas();

    let program = create_program(&context);
    // Generate vertices for screen
    const MIN_X: f32 = -1.0;
    const MAX_X: f32 = 1.0;
    const MIN_Y: f32 = 1.0;
    const MAX_Y: f32 = -1.0;
    const PIXEL_WIDTH: f32 = (MAX_X - MIN_X) / SCREEN_WIDTH as f32;
    const PIXEL_HEIGHT: f32 = (MAX_Y - MIN_Y) / SCREEN_HEIGHT as f32;

    let vertices: Vec<f32> = (0..SCREEN_WIDTH)
        .map(|x| {
            // move here just means copy x
            (0..SCREEN_HEIGHT).map(move |y| {
                // Generate triangles for the pixel at (x, y)
                (0..2)
                    .map(move |x_off| {
                        (0..2)
                            .map(move |y_off| {
                                vec![
                                    MIN_X + PIXEL_WIDTH * (x + x_off) as f32,
                                    MIN_Y + PIXEL_HEIGHT * (y + y_off) as f32,
                                    0.0,
                                ]
                            })
                            .flatten()
                    })
                    .flatten()
            })
        })
        .flatten()
        .flatten()
        .collect();
    // Send vertices to GPU
    create_vertex_array(&context, vertices);

    let indices: Vec<i32> = (0..SCREEN_WIDTH)
        .map(|x| {
            (0..SCREEN_HEIGHT).map(move |y| {
                let i = (x * SCREEN_HEIGHT + y) * 4;
                [0, 1, 2, 1, 2, 3]
                    .map(|j| (j + i) as i32)
                    .into_iter()
                    .collect::<Vec<i32>>()
            })
        })
        .flatten()
        .flatten()
        .collect();
    let indices_len = indices.len();
    // Send indices to GPU
    let indices_buffer = create_buffer_i32(
        &context,
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        indices,
    );
    context.bind_buffer(
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
        Some(&indices_buffer),
    );
    let position_attribute_location = context.get_attrib_location(&program, "position");
    context.vertex_attrib_pointer_with_i32(0 as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    // Send colours to GPU
    let colors = (0..(4 * SCREEN_WIDTH * SCREEN_HEIGHT))
        .map(|i| {
            let c = ((i / 6) % 2) as f32;
            vec![c, c, c]
        })
        .flatten()
        .collect();
    // Create buffer
    let colors_buf = create_buffer_f32(&context, WebGl2RenderingContext::ARRAY_BUFFER, colors);
    // Point GLSL 'color' variable to the array
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&colors_buf));
    let cidx = context.get_attrib_location(&program, "color");
    context.vertex_attrib_pointer_with_i32(
        cidx as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(cidx as u32);

    RenderInfo {
        context,
        colors_buf,
        indices_len,
    }
}

pub fn create_program(context: &WebGl2RenderingContext) -> WebGlProgram {
    // Compile shaders
    let v = compile_shader(
        context,
        WebGl2RenderingContext::VERTEX_SHADER,
        VERTEX_SHADER,
    )
    .unwrap();
    let f = compile_shader(
        context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER,
    )
    .unwrap();

    // Link and return program
    let program = link_program(context, &v, &f).unwrap();
    context.use_program(Some(&program));

    return program;
}

pub fn create_vertex_array(context: &WebGl2RenderingContext, vertices: Vec<f32>) {
    // Create vertex array object
    let vao = context
        .create_vertex_array()
        .expect("Unable to create vertex array!");
    context.bind_vertex_array(Some(&vao));
    // Create buffer
    create_buffer_f32(context, WebGl2RenderingContext::ARRAY_BUFFER, vertices);
}

pub fn create_buffer_f32(
    context: &WebGl2RenderingContext,
    buffer_type: u32,
    data: Vec<f32>,
) -> WebGlBuffer {
    let buf = context.create_buffer().expect("Unable to create buffer!");
    // Transfer data
    buffer_data_f32(context, &buf, buffer_type, data);
    buf
}

pub fn buffer_data_f32(
    context: &WebGl2RenderingContext,
    buf: &WebGlBuffer,
    buffer_type: u32,
    data: Vec<f32>,
) {
    context.bind_buffer(buffer_type, Some(buf));
    unsafe {
        let data_js = js_sys::Float32Array::view(&data);
        context.buffer_data_with_array_buffer_view(
            buffer_type,
            &data_js,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
}

pub fn create_buffer_i32(
    context: &WebGl2RenderingContext,
    buffer_type: u32,
    data: Vec<i32>,
) -> WebGlBuffer {
    let buf = context.create_buffer().expect("Unable to create buffer!");
    context.bind_buffer(buffer_type, Some(&buf));
    unsafe {
        let data_js = js_sys::Int32Array::view(&data);
        context.buffer_data_with_array_buffer_view(
            buffer_type,
            &data_js,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }
    buf
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error compiling shader"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Error compiling shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
