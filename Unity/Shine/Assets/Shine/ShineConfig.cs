using UnityEngine;

namespace Shine
{
    public class ShineConfig : MonoBehaviour
    {
        public const string DLL_PATH_PATTERN_NAME_MACRO = "{name}";
        public const string DLL_PATH_PATTERN_ASSETS_MACRO = "{assets}";
        public const string DLL_PATH_PATTERN_PROJECT_MACRO = "{proj}";

        public ShineConfigOptions Options = new ShineConfigOptions();

        private static ShineConfig singletonInstance_ = null;

        private void OnApplicationQuit()
        {
            ShineGlobalContext.NativeLoader.UnloadAll();
        }

        private void OnEnable()
        {
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

            if (ShineGlobalContext.NativeLoader.NativeLibraryPath != Options.NativeLibraryPath)
            {
                ShineGlobalContext.NativeLoader.UnloadAll();
                ShineGlobalContext.NativeLoader.NativeLibraryPath = Options.NativeLibraryPath;
            }
            ShineGlobalContext.NativeLoader.LoadAll();
        }
    }
}