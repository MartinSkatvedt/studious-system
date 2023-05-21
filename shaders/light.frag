#version 450 core

out vec4 FragColor;

layout(location=1) in vec4 inColor;



void main()
{
    FragColor = inColor;
}



