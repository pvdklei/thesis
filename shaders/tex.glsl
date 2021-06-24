#type vertex
#version 330 core

#define DEFAULT_ATTRIBUTES
#include headers/app.glsl

out struct {
	vec3 normal;
	vec3 fragPosition;
	vec2 UV;
} vs;

void main() {
	gl_Position = app.viewProjection * uModel * vec4(aPos, 1.0);

	vs.fragPosition = aPos;
	vs.normal = aNormal;
	vs.UV = aUV;
}

#type fragment
#version 330 core

#include headers/phong.glsl
#include headers/app.glsl

uniform sampler2D uAlbedoMap;

in struct {
	vec3 normal;
	vec3 fragPosition;
	vec2 UV;
} vs;

out vec4 oFragColor;

void main() {

	vec4 albedo = texture(uAlbedoMap, vs.UV);
	vec3 normal = normalize(vs.normal);

	vec3 lightDir = normalize(vs.fragPosition - app.pointLights[0].position);
	vec3 eyeDir = normalize(app.eyePosition - vs.fragPosition);

	oFragColor = phong(normal, albedo, lightDir, eyeDir, app.pointLights[0].color,
					   uMaterial.reflectiveness, uMaterial.ambient, uMaterial.specular);
}
