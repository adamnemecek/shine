

#[derive(Copy, Clone, Debug, GLShaderDeclaration)]
#[vert_src = "
    attribute vec3 vPosition;
    attribute vec3 vColor;
    uniform mat4 uModelViewProj;
    varying vec3 color;
    void main()
    {
        gl_Position = uModelViewProj * vec4(vPosition, 1.0);
        color = vColor;
    }"]
#[frag_src = "
    varying vec3 color;
    void main()
    {
    	gl_FragColor = vec4(color, 1.);
    }"]
pub struct CubeShader {}
