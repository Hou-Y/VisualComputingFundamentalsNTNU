#version 430 core

in vec4 vertexColor; //get color from vertex shader
in vec3 v_normals; //get normal from vertex shader
out vec4 color;

vec3 lightDirection = normalize(vec3(0.8f, -0.5f, 0.6f));

void main()
{
    //color= vertexColor;

    // Map normals x y z to colours r g b directly
    color = vec4(v_normals, 1.0f); //now it works

    //color = vec4(0.0f, 1.0f, 0.0f, 1.0f);

    // Lamberts cosine law: diffuse factor
    //float diffuse = max(dot(normalize(v_normals), -lightDirection), 0.0f); //with normalization
    //I don't see the difference with or without normalization
    float dif_test = max(dot(v_normals, -lightDirection), 0.0f); //without normalization

    // Apply diffuse only to RGB, keep alpha as is
    //vec3 lambertian_color = vertexColor.rgb * diffuse;
    vec3 lambertian_color = vertexColor.rgb * dif_test;

    color = vec4(lambertian_color, vertexColor.a);

}

//Assuming every object reflects light equally much in all directions ( Lambertian model )
