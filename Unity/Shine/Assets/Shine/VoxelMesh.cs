using System;
using UnityEngine;

namespace Shine
{
    class VoxelMesh : IDisposable
    {
        private IntPtr mesh_;
        private static ShineApi ShineApi = ShineConfig.NativeLoader.LoadNativeLibrary<ShineApi>();

        public VoxelMesh()
        {
            mesh_ = ShineApi.create_mesh();
        }

        public void Dispose()
        {
            if (mesh_ != IntPtr.Zero)
            {
                ShineApi.release_mesh(mesh_);
                mesh_ = IntPtr.Zero;
            }
        }

        public void PolygonizeVoxel(Mesh targetMesh)
        {
            ShineApi.polygonize_voxel(mesh_);
            CopyMesh(targetMesh);
        }

        public void CopyMesh(Mesh targetMesh)
        {
            var info = ShineApi.get_mesh_info(mesh_);
            var positions = new Vector3[info.vertexCount];
            var normals = new Vector3[info.vertexCount];
            var triangles = new int[info.triangleCount];

            unsafe
            {
                fixed (Vector3* posPtr = positions, normPtr = normals)
                {
                    fixed (int* indexPtr = triangles)
                    {
                        ShineApi.fill_mesh_data(mesh_, posPtr, positions.Length, normPtr, normals.Length, indexPtr, triangles.Length);
                    }
                }
            }

            targetMesh.vertices = positions;
            targetMesh.normals = normals;
            targetMesh.triangles = triangles;
        }
    }

}
