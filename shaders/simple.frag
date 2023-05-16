#version 450 core

out vec4 color;

layout(location=3) in vec4 inColor;

void main()
{
    color=inColor;
}