import os

def rename_screens():
    path = '../textures/screenshots'
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
    n = len(os.listdir('../textures/flashback'))
    content = """[
    {
        "type": "skybox",
        "color": "skybox/skybox_night.jpg"
    },
    {
        "type": "camera",
        "pos": [-3.5, 0, 0],
        "dir": [1, 0, 0],
        "fov": 70
    }"""

    for i in range(n):
        content += ',\n    {\n'
        content += '        "type": "rectangle",\n'
        content += f'        "pos": [4, 0, {i * 10}],\n'
        content += '        "dir_w": [0, 0, -1],\n'
        content += '        "dir_l": [0, 1, 0],\n'
        content += '        "length": 4.5,\n'
        content += '        "width": 8,\n'
        content += f'        "color": "flashback/{i + 1}.png"\n'
        content += '    }'

    content += '\n]'


    with open('../scenes/flashback.json', 'w') as file:
        file.seek(0)
        file.write(content)

if __name__ == '__main__':
    # rename_screens()
    generate_flashback()