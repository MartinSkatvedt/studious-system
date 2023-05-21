#version 450 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;

layout(location=1) out vec4 color_out;

uniform layout(location=4) mat4 transform_matrix;

void main()
{
    const vec4 transformed_pos = vec4(position, 1) * transform_matrix;
    gl_Position = transformed_pos;
    
    color_out = color;
}