#type vertex

#version 330 core

#define DEFAULT_ATTRIBUTES
#include headers/app.glsl

out struct {
	vec3 fragPosition;
	vec2 UV;
	mat3 tangentToWorldSpace;
} vs;

void main() {
	gl_Position = app.viewProjection * uModel * vec4(aPos, 1.0);

	vs.tangentToWorldSpace = mat3(aTangent, aBiTangent, aNormal); // Model transform must still be added
	vs.fragPosition = aPos;
	vs.UV = aUV;
}

#type fragment

#version 330 core

#include headers/phong.glsl
#include headers/app.glsl

uniform sampler2D uNormalMap;

in struct {
	vec3 fragPosition;
	vec2 UV;
	mat3 tangentToWorldSpace;
} vs;

out vec4 oFragColor;

void main() {
	vec4 normalM = texture(uNormalMap, vs.UV);
	vec3 normal = normalM.xyz * 2. - 1.;
	normal = normalize(vs.tangentToWorldSpace * normal);

	vec3 lightDir = normalize(vs.fragPosition - app.pointLights[0].position);
	vec3 eyeDir = normalize(app.eyePosition - vs.fragPosition);

	oFragColor = phong(normal, uMaterial.albedo, lightDir, eyeDir, app.pointLights[0].color,
					   uMaterial.reflectiveness, uMaterial.ambient, uMaterial.specular);
}
