using UnityEditor;
using UnityEngine;

namespace Shine
{
    [CustomEditor(typeof(ShineConfig))]
    public class ShineConfigEditor : Editor
    {
        private readonly GUIContent DLL_PATH_PATTERN_GUI_CONTENT = new GUIContent("Library path pattern",
            "Available macros:\n\n" +
            $"{ShineConfig.DLL_PATH_PATTERN_NAME_MACRO} - name of the library\n\n" +
            $"{ShineConfig.DLL_PATH_PATTERN_ASSETS_MACRO} - assets folder of current project.\n\n" +
            $"{ShineConfig.DLL_PATH_PATTERN_PROJECT_MACRO} - project folder i.e. one above Assets.");

        //private readonly GUIContent LINUX_DLOPEN_FLAGS_GUI_CONTENT = new GUIContent("dlopen flags",
        //    $"Flags used in dlopen() P/Invoke on Linux systems. Has minor meaning unless library is large.");

        public override void OnInspectorGUI()
        {
            var t = (ShineConfig)this.target;

            NativeLoader.NativeLibraryPath = EditorGUILayout.TextField(DLL_PATH_PATTERN_GUI_CONTENT, NativeLoader.NativeLibraryPath);
            if (EditorApplication.isPlaying)
            {
                GUI.enabled = false;
            }
            GUI.enabled = true;

            EditorGUILayout.Space();

            var libInfos = NativeLoader.GetInfo();
            if (!EditorApplication.isPaused)
            {
                if (GUILayout.Button("Pause & Unload all libraries"))
                {
                    EditorApplication.isPaused = true;
                    NativeLoader.UnloadAll();
                }
            }
            else
            {
                if (GUILayout.Button("Unpause & Load all libraries"))
                {
                    EditorApplication.isPaused = false;
                }
            }
        }
    }

    [InitializeOnLoadAttribute]
    public static class ShineConfigPauseStateChangedHandler
    {
        static ShineConfigPauseStateChangedHandler()
        {
            EditorApplication.pauseStateChanged += OnPauseStateChanged;
        }

        private static void OnPauseStateChanged(PauseState pauseState)
        {
            Debug.LogError(pauseState);
            if (pauseState == PauseState.Unpaused)
                NativeLoader.LoadAll();
        }
    }
}