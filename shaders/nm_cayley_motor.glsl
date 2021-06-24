
#type vertex

#version 330 core

#include headers/klein.glsl
#include headers/ppga.glsl

#define DEFAULT_ATTRIBUTES
#include headers/app.glsl

out struct {
	vec3 fragPosition;
	vec2 UV;
	vec3 tangentToWorldSpaceEBivector;
	vec3 tangentToWorldSpaceVBivector;
	// ppga_motor tangentToWorldSpaceMotor;
} vs;

void main() {
	ppga_motor tangentToWorldSpaceMotor = ppga_cayley_exp(aTangentToModelSpaceCayleyEBivector, 
												   		  aTangentToModelSpaceCayleyVBivector);
	vec3 pos = ppga_apply_motor_to_origin(tangentToWorldSpaceMotor);
	gl_Position = app.viewProjection * uModel * vec4(pos, 1.0);

	vs.fragPosition = pos;
	vs.UV = aUV;
	// vs.tangentToWorldSpaceMotor = tangentToWorldSpaceMotor; // Model tranform not yet applied
	vs.tangentToWorldSpaceEBivector = aTangentToModelSpaceCayleyEBivector;
	vs.tangentToWorldSpaceVBivector = aTangentToModelSpaceCayleyVBivector;
}

#type fragment

#version 330 core

#include headers/phong.glsl
#include headers/app.glsl
#include headers/klein.glsl
#include headers/ppga.glsl

uniform sampler2D uNormalMap;

in struct {
	vec3 fragPosition;
	vec2 UV;
	vec3 tangentToWorldSpaceEBivector;
	vec3 tangentToWorldSpaceVBivector;
	// ppga_motor tangentToWorldSpaceMotor;
} vs;

out vec4 oFragColor;

void main() {
	vec4 normalM = texture(uNormalMap, vs.UV);
	vec3 normal = normalM.xyz * 2. - 1.;

	ppga_motor tangentToWorldSpaceMotor = ppga_cayley_exp(vs.tangentToWorldSpaceEBivector,
														  vs.tangentToWorldSpaceVBivector);
	
	normal = ppga_apply_motor_to_direction(tangentToWorldSpaceMotor, normal);
	normal = normalize(normal);

	vec3 lightDir = normalize(vs.fragPosition - app.pointLights[0].position);
	vec3 eyeDir = normalize(app.eyePosition - vs.fragPosition);

	oFragColor = phong(normal, uMaterial.albedo, lightDir, eyeDir, app.pointLights[0].color,
					   uMaterial.reflectiveness, uMaterial.ambient, uMaterial.specular);
}
