using UnityEditor;
using UnityEngine;

namespace Shine
{

    public class ShineConfig : MonoBehaviour
    {
        public const string DLL_PATH_PATTERN_NAME_MACRO = "{name}";
        public const string DLL_PATH_PATTERN_ASSETS_MACRO = "{assets}";
        public const string DLL_PATH_PATTERN_PROJECT_MACRO = "{proj}";

        private void OnApplicationQuit()
        {
            NativeLoader.UnloadAll();
        }

        private void OnEnable()
        {
            if (!EnsureUnique())
                return;

            NativeLoader.LoadAll();
        }

        private static ShineConfig singletonInstance_ = null;

        private bool EnsureUnique()
        {
            if (singletonInstance_ != null)
            {
                if (singletonInstance_ != this)
                {
                    Destroy(gameObject);
                }
                return false;
            }
            singletonInstance_ = this;

            DontDestroyOnLoad(gameObject);
            return true;
        }
    }
}