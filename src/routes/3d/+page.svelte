<script>
  import Cube from "../../components/Misc/Cube.svelte";
  import StlViewer from "../../components/Misc/STLViewer.svelte";
</script>

<div class="w-full rounded-2xl overflow-hidden max-w-2xl mx-auto">
  <StlViewer
    stlPath="/chair.stl"
    backgroundColor="#e0e0e0"
    texturePaths={{
      diffuse: "/textures/chair/base.jpg",
      normal: "/textures/chair/normal.jpg",
      roughness: "/textures/chair/roughness.jpg",
    }}
    fallbackColor="#8B4513"
    initialRotation={{ x: 90, y: 180, z: 180 }}
  />
</div>
