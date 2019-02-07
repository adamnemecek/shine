using System;
using System.Runtime.InteropServices;
using UnityEngine;

#pragma warning disable 649 // Field '...' is never assigned to, and will always have its default value null


namespace Shine
{
    internal class ShineApi
    {
        public struct MeshInfo
        {
            public int vertexCount;
            public int triangleCount;
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate IntPtr create_mesh_delegate();
        public create_mesh_delegate create_mesh;

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void release_mesh_delegate(IntPtr meshId);
        public release_mesh_delegate release_mesh;

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate MeshInfo get_mesh_info_delegate(IntPtr meshId);
        public get_mesh_info_delegate get_mesh_info;

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public unsafe delegate void fill_mesh_data_delegate(IntPtr meshId, Vector3* posArray, int posCount, int* indexArray, int indexCount);
        public fill_mesh_data_delegate fill_mesh_data;
    }
}
