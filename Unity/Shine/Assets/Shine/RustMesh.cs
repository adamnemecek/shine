using System;
using UnityEngine;

namespace Shine
{


    class RustMesh : IDisposable
    {
        private IntPtr mesh;
        private static ShineApi ShineApi = NativeLoader.LoadNativeLibrary<ShineApi>();

        public RustMesh()
        {
            mesh = ShineApi.create_mesh();
        }

        public void Dispose()
        {
            if (mesh != IntPtr.Zero)
            {
                ShineApi.release_mesh(mesh);
                mesh = IntPtr.Zero;
            }

        }

        public void CopyMesh(Mesh targetMesh)
        {
            var info = ShineApi.get_mesh_info(mesh);
            var vertices = new Vector3[info.vertexCount];
            var triangles = new int[info.triangleCount];

            unsafe
            {
                fixed (Vector3* posPtr = vertices)
                {
                    fixed (int* indexPtr = triangles)
                    {
                        ShineApi.fill_mesh_data(mesh, posPtr, vertices.Length, indexPtr, triangles.Length);
                    }
                }
            }

            targetMesh.vertices = vertices;
            targetMesh.triangles = triangles;
        }
    }

}
