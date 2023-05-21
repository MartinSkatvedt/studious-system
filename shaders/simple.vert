#version 450 core

layout(location=0) in vec3 position;
layout(location=1) in vec4 color;
layout(location=2) in vec3 normalVector;
layout(location=3) out vec4 outColor;

uniform layout(location=4) mat4 transform_matrix;
uniform layout(location=5) vec4 light_color;
uniform layout(location=6) vec3 light_direction;
uniform layout(location=7) mat4 model_matrix;


void main()
{
    const vec4 transformed_pos = vec4(position, 1) * transform_matrix;
    gl_Position = transformed_pos;

    mat3 scale_rotate_matrix = mat3(model_matrix);

    vec4 ambient = vec4(0.2, 0.2, 0.2, 1.0) * color;
    vec4 diffuse = vec4(max(0, dot(-normalize(normalVector * scale_rotate_matrix), light_direction)) * light_color.rgb, light_color.a);
    
    outColor = ambient + diffuse; 
}