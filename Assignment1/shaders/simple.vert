#version 430 core

in vec3 position;
in mat4 persp;
in layout(location=1) vec4 colors;
in layout(location=2) vec3 normals;

out vec4 vertexColor; //pass data to fragment shader
out vec3 v_normals; //directly pass normals to fragment shader


// Declare a 4x4 matrix inside the shader
//mat4 myMatrix;

uniform mat4 view_projection;  
uniform mat4 model;

void main()
{
    //vec3 flip_pos = vec3(-position.x, -position.y, position.z);
    //gl_Position = vec4(flip_pos, 1.0f);
    vertexColor = colors;

    //v_normals = normals;
    v_normals = normalize(mat3(model)*normals);
    //v_normals = normal_matrix * normals; //correct normal transformation

    //myMatrix = mat4(1.0);
    //initialize matrix
    //myMatrix[0] = vec4(1.0, 0.0, 0.0, 0.0); // column 0
    //myMatrix[1] = vec4(0.0, 1.0, 0.0, 0.0); // column 1
    //myMatrix[2] = vec4(0.0, 0.0, 1.0, 0.0); // column 2
    //myMatrix[3] = vec4(0.8, 0.0, 0.0, 1.0); // column 3

    //gl_Position = myMatrix * vec4(position, 1.0f);
    gl_Position = view_projection * vec4(position, 1.0f);
    //gl_Position = view_projection * model * vec4(position, 1.0f); //wrong!
     

}