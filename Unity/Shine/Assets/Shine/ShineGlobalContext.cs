using System;
using UnityEditor;

namespace Shine
{
    [Serializable]
    public class ShineConfigOptions
    {
        public string NativeLibraryPath { get; set; } =
#if UNITY_STANDALONE_WIN
            "{devel}/{name}.dll";
#elif UNITY_STANDALONE_LINUX
            "{assets}/Plugins/{name}.so";
#endif
    }

    [InitializeOnLoad]
    public static class ShineGlobalContext
    {
        public static NativeLoader NativeLoader = new NativeLoader();
    }
}