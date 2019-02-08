using System;
using UnityEngine;

namespace Shine
{
    [Serializable]
    public class ShineConfigOptions
    {
        public string NativeLibraryPath { get; set; }
    }

    public class ShineConfig : MonoBehaviour
    {
        public const string DLL_PATH_PATTERN_NAME_MACRO = "{name}";
        public const string DLL_PATH_PATTERN_ASSETS_MACRO = "{assets}";
        public const string DLL_PATH_PATTERN_PROJECT_MACRO = "{proj}";

        public static NativeLoader NativeLoader { get; private set; } = new NativeLoader();
        private static ShineConfig singletonInstance_ = null;

        public string NativeLibraryPath = NativeLoader.NativeLibraryPath;

        private void OnApplicationQuit()
        {
            NativeLoader.UnloadAll();
        }

        private void OnEnable()
        {
            Debug.Log($"OnEnable: {NativeLibraryPath}");

            // ensure uniqueness
            if (singletonInstance_ != null)
            {
                if (singletonInstance_ != this)
                {
                    Destroy(gameObject);
                }
                return;
            }
            singletonInstance_ = this;
            DontDestroyOnLoad(gameObject);

            if(NativeLibraryPath != NativeLoader.NativeLibraryPath)
            {
                NativeLoader.UnloadAll();
                NativeLoader.NativeLibraryPath = NativeLibraryPath;
            }
            NativeLoader.LoadAll();
        }
    }
}