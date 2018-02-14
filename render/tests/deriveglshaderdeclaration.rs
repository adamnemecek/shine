extern crate shine_render_core as core;
extern crate shine_render_gl;

use core::*;

#[derive(Copy, Clone, Debug)]
#[derive(GLShaderDeclaration)]
#[vert_path = "fun.glsl"]
#[vert_src = "
    attribute vec3 vPosition;
    attribute vec3 vColor;
    attribute vec2 vTexCoord;
    uniform mat4 uTrsf;
    uniform vec3 uColor;
    varying vec3 color;
    varying vec2 txCoord;

    vec3 col_mod(vec3 c);

    void main()
    {
        color = col_mod(uColor * vColor);
        txCoord = vTexCoord.xy;
        gl_Position = uTrsf * vec4(vPosition, 1.0);
    }"]
#[frag_src = "
    varying vec3 color;
    varying vec2 txCoord;
    uniform sampler2D uTex;
    void main()
    {
        float intensity = texture2D( uTex, txCoord ).r;
        vec3 col =  color * intensity;
        gl_FragColor = vec4(col, 1.0);
    }"]
#[depth = "disable"]
#[cull = "ccw"]
//todo
//#[unifrom(uTrsf = "engine.trsf")]
struct ShSimple {}