import random
from sys import argv

scene_path = "scenes/"
scene_name = "scene.json"

sphere_nb = 10
cylinder_nb = 10
cone_nb = 10
plane_nb = 0

x_max = 100
y_max = 100
z_max = 100

radius_max = 3
height_max = 10
emissive_max = 2
emissive_ratio = 1.0 # 0..1 (ex: 0.3 -> 30% of elements will be emissive)

def randomColor():
    random_r = random.randint(0, 255)
    random_g = random.randint(0, 255)
    random_b = random.randint(0, 255)
    return f"[{random_r}, {random_g}, {random_b}]"

def randomPos():
    random_x = x_max * 2 * random.random() - x_max
    random_y = y_max * 2 * random.random() - y_max
    random_z = z_max * 2 * random.random() - z_max
    return f"[{random_x}, {random_y}, {random_z}]"

def randomDir():
    random_x = 2 * random.random() - 1
    random_y = 2 * random.random() - 1
    random_z = 2 * random.random() - 1
    return f"[{random_x}, {random_y}, {random_z}]"

def randomEmissive():
    if random.random() < emissive_ratio:
        return random.random() * emissive_max
    return 0

def randomParameter(max):
    return random.random() * max


def generateSphere():
    # Generate a sphere

    return f""",
    {{
        "type": "sphere",
        "pos": {randomPos()},
        "dir": {randomDir()},
        "radius": {randomParameter(radius_max)},
        "color": {randomColor()},
        "metalness": {randomParameter(1)},
        "roughness": {randomParameter(1)},
        "emissive": {randomEmissive()}
    }}"""

def generateCylinder():
    # Generate a cylinder

    return f""",
    {{
        "type": "cylinder",
        "pos": {randomPos()},
        "dir": {randomDir()},
        "radius": {randomParameter(radius_max)},
        "height": {randomParameter(height_max)},
        "color": {randomColor()},
        "metalness": {randomParameter(1)},
        "roughness": {randomParameter(1)},
        "emissive": {randomEmissive()}
    }}"""

def generateCone():
    # Generate a cone

    return f""",
    {{
        "type": "cone",
        "pos": {randomPos()},
        "dir": {randomDir()},
        "radius": {randomParameter(radius_max)},
        "height": {randomParameter(height_max)},
        "color": {randomColor()},
        "metalness": {randomParameter(1)},
        "roughness": {randomParameter(1)},
        "emissive": {randomEmissive()}
    }}"""

def generatePlane():
    # Generate a plane

    return f""",
    {{
        "type": "plane",
        "pos": {randomPos()},
        "dir": {randomDir()},
        "color": {randomColor()},
        "metalness": {randomParameter(1)},
        "roughness": {randomParameter(1)},
        "emissive": {randomEmissive()}
    }}"""

def generateScene():
    # Generate a scene
    scene_str = f'''[
    {{
        "type": "camera",
        "pos": [0, 0, -10],
        "dir": [0, 0, 1],
        "fov": 60
    }}'''
    
    for i in range(sphere_nb):
        scene_str += generateSphere()

    for i in range(cylinder_nb):
        scene_str += generateCylinder()

    for i in range(cone_nb):
        scene_str += generateCone()

    for i in range(plane_nb):
        scene_str += generatePlane()

    scene_str += "\n]"

    with open(scene_path + scene_name, "w") as f:
        f.write(scene_str)


if __name__ == "__main__":
    if len(argv) > 1:
        scene_name = argv[1]
    if len(argv) > 2:
        sphere_nb = int(argv[2])
        cylinder_nb = int(argv[2])
        cone_nb = int(argv[2])
    if len(argv) > 3:
        plane_nb = int(argv[3])

    if len(argv) == 1:
        print("Tips: You can also provide args!\npython3 generate_scene.py [scene_name] [nb_objects] [nb_planes]\ndefaults are \"scene.json\", 10, 0")

    generateScene()
