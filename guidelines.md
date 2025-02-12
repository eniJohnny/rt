### **Preliminaries**  
Before evaluation begins, ensure:  
- [ ] **Submission Verification**: The project is in the correct Git repository.  
- [ ] **Team Presence**: All group members are present.  
- [ ] **Program Stability**: The program does not crash or terminate unexpectedly.  

---

### **Mandatory Part**  
Failure to complete this section results in an automatic fail.  

#### **1. Basic Ray Tracing Functionality**  
- [ ] The program renders three required scenes correctly.  

#### **2. Window Exposure Handling**  
- [ ] The window does not unnecessarily recompute the scene when focus changes.  
- [ ] `mlx_expose_hook` (or equivalent) is correctly implemented.  

#### **3. Object Rendering**  
- [ ] Supports the four basic objects: **Sphere, Plane, Cylinder, Cone**.  
- [ ] Multiple objects of the same type can coexist.  
- [ ] Objects can be translated and rotated.  
- [ ] Each object has an independent intersection function.  

#### **4. Camera Placement & Viewing**  
- [ ] The camera can move and rotate freely.  
- [ ] The second required scene can be achieved by modifying camera position.  

#### **5. Lighting and Shadows**  
- [ ] Brightness gradients are correctly implemented.  
- [ ] Shadows are cast realistically.  
- [ ] Specular highlights (shiny reflections) are visible.  
- [ ] Multiple light sources interact correctly.  

---

### **Optional Features (Additional Points)**  
Only evaluated if the **mandatory part** is fully completed.  

#### **1. Scene Files**  
- [ ] Scene descriptions are stored in structured files (e.g., XML).  

#### **2. Ambient Lighting**  
- [ ] Objects are never fully black.  
- [ ] Ambient light can be adjusted via configuration file.  

#### **3. Sliced Objects**  
- [ ] Objects can be sliced along the X, Y, Z axes.  
- [ ] Slices can be adjusted independently for each object.  
- [ ] Rotation and translation still work after slicing.  

#### **4. Disturbances (Pattern & Texture Disruptions)**  
- [ ] **Normal disturbances** (e.g., sine waves for wavy effects).  
- [ ] **Color disruptions** (checkerboard, noise-based textures).  

#### **5. Advanced Lighting**  
- [ ] Direct light sources cause glare when facing them.  
- [ ] Parallel light sources emit in a fixed direction.  

#### **6. Reflection & Transparency**  
- [ ] Reflection is implemented.  
- [ ] Reflection intensity can be adjusted.  
- [ ] Transparency is implemented.  
- [ ] Refraction index is correctly calculated (Snell’s Law).  
- [ ] Transparency intensity can be adjusted.  

#### **7. Shadows with Transparency**  
- [ ] Shadows become lighter when cast by transparent objects.  

#### **8. Texturing**  
- [ ] At least one basic object supports textures.  
- [ ] All four basic objects can have textures.  
- [ ] Textures can be stretched, shifted, or scaled.  
- [ ] Non-MinilibX formats (PNG, JPEG) are supported.  

#### **9. Advanced Texture Effects**  
- [ ] **Bump mapping** (textures modify surface normals).  
- [ ] **Transparency mapping** (textures affect object transparency).  
- [ ] **Texture-based slicing** (textures determine object visibility).  
- [ ] **Projected textures** (like a video projector).  

#### **10. Composite Objects**  
- [ ] Objects can be built from multiple primitives.  
- [ ] Instances of composite objects support independent transformations.  

#### **11. Negative Objects (CSG - Constructive Solid Geometry)**  
- [ ] Objects can be subtracted from one another.  
- [ ] Object intersections modify geometry dynamically.  

#### **12. Advanced Rendering Techniques**  
- [ ] **Antialiasing** (smooth edges).  
- [ ] **Cartoon shading** (flat colors with outlines).  
- [ ] **Motion blur** (objects blur when moving).  
- [ ] **Color filters** (sepia, grayscale).  
- [ ] **Stereoscopic 3D** (red-green glasses effect).  

#### **13. Performance Optimization**  
- [ ] Multi-threading is used for faster rendering.  
- [ ] Distributed computing spreads computation across multiple systems.  
- [ ] The program renders images efficiently.  
- [ ] Screenshots can be saved.  

#### **14. Graphical User Interface & Interaction**  
- [ ] **Basic UI** (progress bars, status messages).  
- [ ] **Advanced UI** (GTK or QT interface with menus).  
- [ ] **Live scene interaction** (adjust camera or objects dynamically).  
- [ ] **Batch rendering** (automated multi-frame output).  

---

### **Final Evaluation**  

#### **1. Group Organization**  
- [ ] The team managed time and tasks effectively.  

#### **2. Creativity & Additional Features**  
- [ ] **Exotic Objects** (torus, fractals, unique shapes).  
- [ ] **VR/3D TV Support** (headsets, 3D models).  
- [ ] **Special Rendering Effects** (caustics, global illumination).  
- [ ] **Creative Experimentation** (e.g., Möbius strip rendering).  

#### **3. Overall Aesthetic Appeal**  
- [ ] The rendered images look visually appealing.  

---

### **Final Ratings & Comments**  
- [ ] The project meets all basic and advanced requirements.  
- [ ] The final render is **exceptional**.  

---