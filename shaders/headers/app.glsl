#ifndef APP_GLSL
#define APP_GLSL

#define MAX_POINT_LIGHTS 1

struct PointLight {
	vec3 position;
	vec3 color;
};

struct DirLight {
	vec3 direction;
	vec3 color;
};

struct Material {
	vec4 albedo;
	float ambient;
	float specular;
	int reflectiveness;
};

layout (std140) uniform App {
	mat4 viewProjection;
	mat4 view;
	mat4 projection;
	mat4 orthoProj;
	vec3 eyePosition;   
	PointLight[MAX_POINT_LIGHTS] pointLights;
} app; 

uniform Material uMaterial = Material(vec4(1., 1., 0., 1.), 0.5, 0.5, 32);
uniform mat4 uModel = mat4(1, 0, 0, 0,
						   0, 1, 0, 0,
						   0, 0, 1, 0,
						   0, 0, 0, 1);


#ifdef NORMTANG_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec3 aTangent;
#endif

#ifdef MATRIX_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBiTangent;
#endif

#ifdef ROTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec4 aTangentToModelSpaceRotor;
#endif

#ifdef OUTER_ROTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aTangentToModelSpaceOuterRotor;
#endif

#ifdef CAYLEY_ROTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aTangentToModelSpaceCayleyRotor;
#endif

#ifdef QROTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aTangentToModelSpaceQTang;
#endif

#ifdef MOTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec4 aTangentToModelSpaceMotor1;
layout (location = 3) in vec4 aTangentToModelSpaceMotor2;
#endif

#ifdef OUTER_MOTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aTangentToModelSpaceOuterEBivector;
layout (location = 3) in vec3 aTangentToModelSpaceOuterVBivector;
#endif

#ifdef CAYLEY_MOTOR_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aTangentToModelSpaceCayleyEBivector;
layout (location = 3) in vec3 aTangentToModelSpaceCayleyVBivector;
#endif

#ifdef DEFAULT_ATTRIBUTES
#undef DEFAULT_ATTRIBUTES
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUV;
layout (location = 2) in vec3 aNormal;
layout (location = 3) in vec3 aTangent;
layout (location = 4) in vec3 aBiTangent;
layout (location = 5) in vec4 aTangentToModelSpaceRotor;
layout (location = 6) in vec4 aTangentToModelSpaceMotor1;
layout (location = 7) in vec4 aTangentToModelSpaceMotor2;
layout (location = 8) in vec3 aTangentToModelSpaceOuterEBivector;
layout (location = 9) in vec3 aTangentToModelSpaceOuterVBivector;
layout (location = 10) in vec3 aTangentToModelSpaceOuterRotor;
layout (location = 11) in vec3 aTangentToModelSpaceQTang;
layout (location = 12) in vec3 aTangentToModelSpaceCayleyEBivector;
layout (location = 13) in vec3 aTangentToModelSpaceCayleyVBivector;
layout (location = 14) in vec3 aTangentToModelSpaceCayleyRotor;
#endif

#ifdef TANGENT_MOTOR
#undef TANGENT_MOTOR
ppga_motor aTangentToModelSpaceMotor() {
	return ppga_motor(aTangentToModelSpaceMotor1, aTangentToModelSpaceMotor2);
}
#endif 

#endif // GUARD
