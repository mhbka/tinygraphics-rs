#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;

uniform vec3 viewPos;
uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;
uniform mat4 modelNorm;

out vec3 fragPos;
out vec3 fragNormal;
out vec3 viewerPos;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    fragPos = vec3(model * vec4(aPos, 1.0));
    fragNormal = normalize(aNormal) * mat3(modelNorm);
    viewerPos = viewPos;
}