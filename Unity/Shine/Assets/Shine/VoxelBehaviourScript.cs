using Shine;
using UnityEngine;


[RequireComponent(typeof(MeshFilter))]
public class VoxelBehaviourScript : MonoBehaviour
{
    private VoxelMesh mesh_;

    // Start is called before the first frame update
    void Start()
    {
        mesh_ = new VoxelMesh();
        Mesh mesh = new Mesh();
        GetComponent<MeshFilter>().mesh = mesh;
        mesh_.PolygonizeVoxel(mesh);
    }

    // Update is called once per frame
    void Update()
    {

    }
}

