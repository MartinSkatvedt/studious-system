#version 450 core

layout(location=0) in vec3 position;
layout(location=2) in vec3 normalVector;

layout(location=2) out vec3 normal_vector_out;
layout(location=3) out vec3 frag_pos_out;

uniform layout(location=4) mat4 transform_matrix;
uniform layout(location=7) mat4 model_matrix;

void main()
{
    const vec4 transformed_pos = vec4(position, 1) * transform_matrix;
    gl_Position = transformed_pos;
    
    frag_pos_out = vec3(vec4(position, 1) * model_matrix);
    normal_vector_out = normalVector;
}