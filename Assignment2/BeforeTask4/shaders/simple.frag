#version 430 core

in vec4 vertexColor; //get data from vertex shader
out vec4 color;

void main()
{
    color= vertexColor;
    //color = vec4(0.0f, 1.0f, 0.0f, 1.0f);
}
