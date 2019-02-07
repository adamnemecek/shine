using UnityEditor;
using UnityEngine;

namespace Shine
{
    [InitializeOnLoadAttribute]
    public static class ShineConfig {
        public const string DLL_PATH_PATTERN_NAME_MACRO = "{name}";
        public const string DLL_PATH_PATTERN_ASSETS_MACRO = "{assets}";
        public const string DLL_PATH_PATTERN_PROJECT_MACRO = "{proj}";

        public static NativeLoader NativeLoader = new NativeLoader();
    }

    public class ShineConfigProps : MonoBehaviour
    {
        private void OnApplicationQuit()
        {
            ShineConfig.NativeLoader.UnloadAll();
        }

        private void OnEnable()
        {
            ShineConfig.NativeLoader.LoadAll();
        }
    }
}