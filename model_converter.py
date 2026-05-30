import sys

def get_verticies(file_content: str) -> list[tuple[float, float, float]]:
    verticies: list[tuple[float, float, float]] = []
    for line in file_content.split("\n"):
        if line.startswith("v "):
            coordinates = line.split()[1:]
            verticies.append((float(coordinates[0]), float(coordinates[1]), float(coordinates[2])))
    return verticies

def get_faces(file_content: str) -> list[tuple[list, list, list]]:
    faces: list[tuple[list, list, list]] = []
    for line in file_content.split("\n"):
        if line.startswith("f "):
            vertex = line.split()[1:]
            faces.append((vertex[0].split("/"), vertex[1].split("/") , vertex[2].split("/")))
    return faces

def get_texture_coordinates(file_content: str) -> list[tuple[float, float]]:
    texture_coords: list[tuple[float, float]] = []
    for line in file_content.split("\n"):
        if line.startswith("vt "):
            coordinates = line.split()[1:]
            texture_coords.append((float(coordinates[0]), float(coordinates[1])))
    return texture_coords

def compile_verticies(file_content: str):
    verticies = get_verticies(file_content)
    compiled = []
    for vertex in verticies:
        compiled.append(f"    mesh.add_vertex(Vector3::new({vertex[0]}, {vertex[1]}, {vertex[2]}));")
    return compiled

def compile_faces(file_content: str):
    faces = get_faces(file_content)
    texture_coordinates = get_texture_coordinates(file_content)
    compiled = []
    for face in faces:
        v1 = int(face[0][0]) - 1
        v2 = int(face[1][0]) - 1
        v3 = int(face[2][0]) - 1
        t1 = texture_coordinates[int(face[0][1]) - 1]
        t2 = texture_coordinates[int(face[1][1]) - 1]
        t3 = texture_coordinates[int(face[2][1]) - 1]
        compiled.append(f"    mesh.add_triangle(MeshTriangle {{ v1: {v1}, v2: {v2}, v3: {v3}, t1: Vector2::new({t1[0]}, {t1[1]}), t2: Vector2::new({t2[0]}, {t2[1]}), t3: Vector2::new({t3[0]}, {t3[1]})}});")
    return compiled

def main():
    with open(sys.argv[1]) as f:
        file_content: str = f.read()
        out = []
        out.append("use nalgebra::{Vector2, Vector3};")
        out.append("use crate::renderer::mesh::{Mesh, MeshTriangle};")
        out.append("pub fn load_mesh(mesh: &mut Mesh) {")
        out.extend(compile_verticies(file_content))
        out.extend(compile_faces(file_content))
        out.append("}")
    with open(sys.argv[2], "w") as f:
        f.write("\n".join(out))

if __name__ == "__main__":
    main()
