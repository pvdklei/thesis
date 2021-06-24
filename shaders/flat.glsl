#type vertex

#version 330 core

#define DEFAULT_ATTRIBUTES
#include headers/app.glsl

out struct {
	vec3 normal;
	vec3 fragWorldPos;
} vs;

void main() {
	gl_Position = app.viewProjection *  uModel * vec4(aPos, 1.0);
	vs.normal = aNormal;
	vs.fragWorldPos = (uModel * vec4(aPos, 1.0)).xyz;
}

#type fragment

#version 330 core

#include headers/app.glsl
#include headers/phong.glsl

in struct {
	vec3 normal;
	vec3 fragWorldPos;
} vs;

out vec4 oFragColor;

void main() {

	vec3 normal = normalize(vs.normal);
	vec3 lightDir = normalize(vs.fragWorldPos - app.pointLights[0].position);
	vec3 eyeDir = normalize(app.eyePosition - vs.fragWorldPos);
	oFragColor = phong(normal, uMaterial.albedo, lightDir, eyeDir, app.pointLights[0].color,
					   uMaterial.reflectiveness, uMaterial.ambient, uMaterial.specular);
}
