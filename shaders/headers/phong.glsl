#ifndef PHONG_GLSL
#define PHONG_GLSL

vec4 phong(vec3 normal, vec4 albedo, vec3 dirToLight, 
		   vec3 dirToEye, vec3 lightColor, 
		   int reflectiveness, float ambientStrength, float specularStrength) {

	vec3 ambient = ambientStrength * lightColor;

	float lambertian = max(dot(normal, -dirToLight), 0.0);
	vec3 diffuse = lambertian * lightColor;

	vec3 reflectedLightDir = reflect(dirToLight, normal);
	float spec = max(dot(dirToEye, reflectedLightDir), 0.0);
	vec3 specular = pow(spec, reflectiveness) * specularStrength *  lightColor;

	vec4 color = vec4(specular + ambient + diffuse, 1.0) * albedo;
	return color;
}

#endif

