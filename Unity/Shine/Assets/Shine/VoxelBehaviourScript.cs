using Shine;
using UnityEngine;


public class VoxelBehaviourScript : MonoBehaviour
{
    // Start is called before the first frame update
    void Start()
    {
        var rustMesh = new RustMesh();
        Mesh mesh = new Mesh();
        GetComponent<MeshFilter>().mesh = mesh;
        rustMesh.CopyMesh(mesh);
    }

    // Update is called once per frame
    void Update()
    {

    }
}

