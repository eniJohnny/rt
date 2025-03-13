import os

def rename_screens():
    path = 'textures/screenshots'
    files = os.listdir(path)
    files = [file for file in files if file.endswith('.png')]
    
    indexed_files = []
    for file in files:
        index = file.split('_')[0]
        indexed_files.append((int(index), file))

    indexed_files.sort(key=lambda x: x[0])
    indexed_files.reverse()
    
    for i, file in enumerate(indexed_files):
        os.rename(f'{path}/{file[1]}', f'{path}/{i + 1}.png')

def generate_flashback():
    n = len(os.listdir('textures/flashback'))
    content = """[
    {
        "type": "camera",
        "pos": [0, 0, 0],
        "dir": [0, 0, 1],
        "fov": 70
    },
    {
        "type": "plane",
        "pos": [-5, 0, 0],
        "dir": [1, 0, 0],
        "color": [200, 200, 200],
        "roughness": 1.0
    },
    {
        "type": "plane",
        "pos": [5, 0, 0],
        "dir": [-1, 0, 0],
        "color": [200, 200, 200],
        "roughness": 1.0,
        "emissive": 1.0
    },
    {
        "type": "plane",
        "pos": [0, -5, 0],
        "dir": [0, 1, 0],
        "color": [255, 255, 255],
        "roughness": 1.0
    },
    {
        "type": "plane",
        "pos": [0, 5, 0],
        "dir": [0, -1, 0],
        "color": [255, 255, 255],
        "roughness": 1.0
    }"""

    for i in range(n):
        content += ',\n    {\n'
        content += '        "type": "rectangle",\n'
        content += f'        "pos": [4, 0, {i * 20 + 20}],\n'
        content += '        "dir_w": [0.2, 0, -1],\n'
        content += '        "dir_l": [0, 1, 0],\n'
        content += '        "length": 4.5,\n'
        content += '        "width": 8,\n'
        content += f'        "color": "flashback/{i + 1}.png"\n'
        content += '    }'
        content += ',\n    {\n'
        content += '        "type": "rectangle",\n'
        content += f'        "pos": [4.002, 0, {i * 20 + 20.01}],\n'
        content += '        "dir_w": [0.2, 0, -1],\n'
        content += '        "dir_l": [0, 1, 0],\n'
        content += '        "length": 4.7,\n'
        content += '        "width": 8.2,\n'
        content += f'        "color": [0, 0, 0]\n'
        content += '    }'
        content += ',\n    {\n'
        content += '        "type": "rectangle",\n'
        content += f'        "pos": [4.01, 3, {i * 20 + 20}],\n'
        content += '        "dir_w": [0.2, 0, -1],\n'
        content += '        "dir_l": [0, 1, 0],\n'
        content += '        "length": 4,\n'
        content += '        "width": 0.25,\n'
        content += f'        "color": [0, 0, 0]\n'
        content += '    }'

    content += '\n]'

    with open('scenes/flashback.json', 'w') as file:
        file.write(content)

if __name__ == '__main__':
    # rename_screens()
    generate_flashback()