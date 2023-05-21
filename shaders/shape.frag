#version 450 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

out vec4 FragColor;

layout(location=2) in vec3 normalVector;
layout(location=3) in vec3 frag_pos;

uniform layout(location=7) mat4 model_matrix;
uniform layout(location=8) vec3 camera_position;
uniform layout(location=9) Material material;
uniform layout(location=13) Light light;

void main()
{
    mat3 scale_rotate_matrix = mat3(model_matrix);

    vec3 actual_normal = normalize(normalVector * scale_rotate_matrix);
    vec3 light_direction = normalize(light.position - frag_pos);

    //Ambient component
    vec3 ambient = material.ambient * light.ambient;

    //Diffuse component
    vec3 diffuse = (max(0, dot(actual_normal, light_direction)) * material.diffuse) * light.diffuse;

    //Specular component
    vec3 camera_direction = normalize(camera_position - frag_pos);
    vec3 reflection_direction = reflect(-light_direction, actual_normal);
    float spec = pow(max(dot(camera_direction, reflection_direction), 0.0), material.shininess);
    vec3 specular = (material.specular * spec) * light.specular;


    vec3 color =  (ambient + diffuse + specular);
    FragColor = vec4(color, 1.0);
}






