import struct
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
        compiled.append(struct.pack("<3f", vertex[0], vertex[1], vertex[2]))
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
        compiled.append(v1.to_bytes(2, "little"))
        compiled.append(v2.to_bytes(2, "little"))
        compiled.append(v3.to_bytes(2, "little"))
        compiled.append((0).to_bytes(2, "little"))
        compiled.append(struct.pack("<6f", t1[0], 1 - t1[1], t2[0], 1 - t2[1], t3[0], 1 - t3[1]))
    return compiled

def main():
    with open(sys.argv[1]) as f:
        file_content: str = f.read()
        verticies = compile_verticies(file_content)
        faces = compile_faces(file_content)
    with open(sys.argv[2], "wb") as f:
        f.write(b''.join(verticies))
    with open(sys.argv[3], "wb") as f:
        f.write(b''.join(faces))

if __name__ == "__main__":
    main()
