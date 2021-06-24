
#type vertex

#version 330 core
#define DEFAULT_ATTRIBUTES
#include headers/app.glsl

uniform mat4 uModel;

out struct {
	vec3 fragPosition;
	vec2 UV;
	vec4 tangentToWorldSpaceRotor;
} vs;

void main() {
	gl_Position = app.viewProjection * uModel * vec4(aPos, 1.0);

	vs.fragPosition = aPos;
	vs.UV = aUV;
	vs.tangentToWorldSpaceRotor = aTangentToWorldSpaceRotor;
}

#type fragment

#version 330 core

#include headers/phong.glsl
#include headers/app.glsl
#include headers/klein.glsl

uniform sampler2D uNormalMap;

in struct {
	vec3 fragPosition;
	vec2 UV;
	vec4 tangentToWorldSpaceRotor;
} vs;

out vec4 oFragColor;

void main() {
	vec4 normalM = texture(uNormalMap, vs.UV);
	vec3 normal = normalM.xyz * 2. - 1.;
	
	kln_rotor tangentToWorldSpace = kln_rotor(vs.tangentToWorldSpaceRotor);
	kln_point pgaNormal = kln_point(vec4(0, -normal));
	pgaNormal = kln_apply(tangentToWorldSpace, pgaNormal);
	normal = normalize(-pgaNormal.p3.yzw);

	vec3 lightDir = normalize(vs.fragPosition - app.pointLights[0].position);
	vec3 eyeDir = normalize(app.eyePosition - vs.fragPosition);

	oFragColor = phong(normal, uMaterial.albedo, lightDir, eyeDir, app.pointLights[0].color,
					   uMaterial.reflectiveness, uMaterial.ambient, uMaterial.specular);
}
