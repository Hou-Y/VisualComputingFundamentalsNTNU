#version 430 core

in vec3 position;
in layout(location=1) vec4 colors;
out vec4 vertexColor; //pass data to fragment shader

// Declare a 4x4 matrix inside the shader
mat4 myMatrix;

void main()
{
    vec3 flip_pos = vec3(-position.x, -position.y, position.z);
    gl_Position = vec4(flip_pos, 1.0f);
    vertexColor = colors;

    myMatrix = mat4(1.0);
    //initialize matrix
    myMatrix[0] = vec4(1.0, 0.0, 0.0, 0.0); // column 0
    myMatrix[1] = vec4(0.0, 1.0, 0.0, 0.0); // column 1
    myMatrix[2] = vec4(0.0, 0.0, 1.0, 0.0); // column 2
    myMatrix[3] = vec4(0.8, 0.0, 0.0, 1.0); // column 3

    gl_Position = myMatrix * vec4(position, 1.0f);


}