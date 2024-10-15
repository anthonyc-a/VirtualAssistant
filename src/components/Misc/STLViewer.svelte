<script lang="ts">
  import { onMount, afterUpdate, createEventDispatcher } from "svelte";
  import * as THREE from "three";
  import { STLLoader } from "three/examples/jsm/loaders/STLLoader";
  import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
  import {
    CSS2DRenderer,
    CSS2DObject,
  } from "three/examples/jsm/renderers/CSS2DRenderer";
  import TGA from "tga-js";

  export let stlPath: string;
  export let texturePaths: {
    diffuse?: string;
    normal?: string;
    ao?: string;
    roughness?: string;
    metalness?: string;
  } = {};
  export let fallbackColor: string = "#ffffff";
  export let backgroundColor: string = "#333333";

  let container: HTMLDivElement;
  let width: number;
  let height: number;
  let renderer: THREE.WebGLRenderer;
  let labelRenderer: CSS2DRenderer;
  let scene: THREE.Scene;
  let camera: THREE.PerspectiveCamera;
  let controls: OrbitControls;
  let model: THREE.Mesh;

  export let initialRotation: { x: number; y: number; z: number } = {
    x: 0,
    y: 0,
    z: 0,
  };

  let modelStats = {
    vertices: 0,
    faces: 0,
  };

  let cameraPosition = { x: 0, y: 0, z: 0 };
  let zoomLevel = 100;

  let isLoading = true;
  let loadingProgress = 0;

  const dispatch = createEventDispatcher();

  function loadTexture(url: string): Promise<THREE.Texture> {
    if (url.toLowerCase().endsWith(".tga")) {
      return loadTGA(url);
    } else {
      return new Promise((resolve, reject) => {
        new THREE.TextureLoader().load(url, resolve, undefined, reject);
      });
    }
  }

  function loadTGA(url: string): Promise<THREE.Texture> {
    return new Promise((resolve, reject) => {
      const loader = new THREE.FileLoader();
      loader.setResponseType("arraybuffer");
      loader.load(
        url,
        function (buffer) {
          const tga = new TGA();
          tga.load(new Uint8Array(buffer as ArrayBuffer));
          const canvas = tga.getCanvas();
          const texture = new THREE.CanvasTexture(canvas);
          texture.flipY = false;
          resolve(texture);
        },
        undefined,
        reject
      );
    });
  }

  function createHUDElement(
    id: string,
    className: string,
    innerHTML: string
  ): HTMLElement {
    const element = document.createElement("div");
    element.id = id;
    element.className = className;
    element.innerHTML = innerHTML;
    return element;
  }

  function updateHUD() {
    const statsElement = document.getElementById("model-stats");
    if (statsElement) {
      statsElement.innerHTML = `Vertices: ${modelStats.vertices} | Faces: ${modelStats.faces}`;
    }

    const cameraElement = document.getElementById("camera-position");
    if (cameraElement) {
      cameraElement.innerHTML = `Camera: X: ${cameraPosition.x.toFixed(2)} Y: ${cameraPosition.y.toFixed(2)} Z: ${cameraPosition.z.toFixed(2)}`;
    }

    const zoomElement = document.getElementById("zoom-level");
    if (zoomElement) {
      zoomElement.innerHTML = `Zoom: ${zoomLevel.toFixed(0)}%`;
    }
  }

  async function createScene() {
    isLoading = true;
    loadingProgress = 0;

    scene = new THREE.Scene();
    scene.background = new THREE.Color(backgroundColor);

    camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    camera.position.set(0, 0, 5);

    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(width, height);
    renderer.outputEncoding = THREE.sRGBEncoding;
    container.appendChild(renderer.domElement);

    labelRenderer = new CSS2DRenderer();
    labelRenderer.setSize(width, height);
    labelRenderer.domElement.style.position = "absolute";
    labelRenderer.domElement.style.top = "0px";
    container.appendChild(labelRenderer.domElement);

    controls = new OrbitControls(camera, labelRenderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.25;
    controls.enableZoom = true;
    controls.autoRotate = false;
    controls.autoRotateSpeed = 0.5;
    controls.enablePan = true;

    const loader = new STLLoader();

    try {
      const geometry = await new Promise<THREE.BufferGeometry>((resolve) => {
        loader.load(
          stlPath,
          resolve,
          (xhr) => {
            loadingProgress = (xhr.loaded / xhr.total) * 100;
          },
          (error) => {
            console.error("Error loading STL:", error);
            isLoading = false;
          }
        );
      });
      const material = new THREE.MeshStandardMaterial({
        color: new THREE.Color(fallbackColor),
        metalness: 0.5,
        roughness: 0.5,
      });

      for (const [key, path] of Object.entries(texturePaths)) {
        if (path) {
          try {
            const texture = await loadTexture(path);
            switch (key) {
              case "diffuse":
                material.map = texture;
                material.color.setHex(0xffffff);
                break;
              case "normal":
                material.normalMap = texture;
                break;
              case "ao":
                material.aoMap = texture;
                break;
              case "roughness":
                material.roughnessMap = texture;
                break;
              case "metalness":
                material.metalnessMap = texture;
                break;
            }
          } catch (error) {
            console.error(`Failed to load texture: ${key}`, error);
            if (key === "diffuse") {
              material.color.set(fallbackColor);
            }
          }
        }
      }

      material.needsUpdate = true;

      model = new THREE.Mesh(geometry, material);

      geometry.computeBoundingBox();
      const center = new THREE.Vector3();
      geometry.boundingBox!.getCenter(center);
      model.position.sub(center);

      model.rotation.set(
        THREE.MathUtils.degToRad(initialRotation.x),
        THREE.MathUtils.degToRad(initialRotation.y),
        THREE.MathUtils.degToRad(initialRotation.z)
      );

      scene.add(model);

      const box = new THREE.Box3().setFromObject(model);
      const size = box.getSize(new THREE.Vector3());
      const maxDim = Math.max(size.x, size.y, size.z);
      camera.position.z = maxDim * 2;
      controls.target.copy(center);
      controls.update();

      modelStats.vertices = geometry.attributes.position.count;
      modelStats.faces = geometry.index
        ? geometry.index.count / 3
        : geometry.attributes.position.count / 3;
    } catch (error) {
      console.error("Error loading model or textures:", error);
    }

    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);

    const frontLight = new THREE.DirectionalLight(0xffffff, 0.8);
    frontLight.position.set(0, 0, 1);
    scene.add(frontLight);

    const backLight = new THREE.DirectionalLight(0xffffff, 0.3);
    backLight.position.set(0, 0, -1);
    scene.add(backLight);

    const topLight = new THREE.DirectionalLight(0xffffff, 0.3);
    topLight.position.set(0, 1, 0);
    scene.add(topLight);

    // Create HUD elements
    const hudContainer = document.createElement("div");
    hudContainer.className = "hud-container";
    hudContainer.style.position = "absolute";
    hudContainer.style.top = "0";
    hudContainer.style.left = "0";
    hudContainer.style.width = "100%";
    hudContainer.style.height = "100%";
    hudContainer.style.pointerEvents = "none";
    container.appendChild(hudContainer);

    const topLeft = createHUDElement("model-stats", "hud-element top-left", "");
    const topRight = createHUDElement(
      "camera-position",
      "hud-element top-right",
      ""
    );
    const bottomLeft = createHUDElement(
      "zoom-level",
      "hud-element bottom-left",
      ""
    );
    const bottomRight = createHUDElement(
      "controls-info",
      "hud-element bottom-right",
      "Left click: Rotate | Right click: Pan | Scroll: Zoom"
    );

    hudContainer.appendChild(topLeft);
    hudContainer.appendChild(topRight);
    hudContainer.appendChild(bottomLeft);
    hudContainer.appendChild(bottomRight);
  }

  function animate() {
    requestAnimationFrame(animate);
    controls.update();

    cameraPosition = camera.position;
    zoomLevel = 100 / camera.position.distanceTo(controls.target);

    updateHUD();

    renderer.render(scene, camera);
    labelRenderer.render(scene, camera);
  }

  function handleResize() {
    if (container) {
      width = container.clientWidth;
      height = container.clientHeight;
      if (camera) {
        camera.aspect = width / height;
        camera.updateProjectionMatrix();
      }
      if (renderer) {
        renderer.setSize(width, height);
      }
      if (labelRenderer) {
        labelRenderer.setSize(width, height);
      }
    }
  }

  onMount(() => {
    handleResize();
    createScene();
    animate();
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      if (renderer) renderer.dispose();
      if (model) {
        if (model.geometry) model.geometry.dispose();
        if (model.material) {
          if (Array.isArray(model.material)) {
            model.material.forEach((m) => m.dispose());
          } else {
            model.material.dispose();
          }
        }
      }
    };
  });

  afterUpdate(() => {
    handleResize();
  });
</script>

<div bind:this={container} class="w-full h-[480px] rounded-2xl overflow-hidden relative">
  {#if isLoading}
    <div class="loading-bar">
      <div class="loading-progress" style="width: {loadingProgress}%"></div>
    </div>
  {/if}
</div>

<style>
  .hud-container {
    font-family: Arial, sans-serif;
    text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.5);
  }
  .hud-element {
    position: absolute;
    padding: 10px;
    background-color: rgba(0, 0, 0, 0.5);
    border-radius: 5px;
  }
  .top-left {
    top: 10px;
    left: 10px;
  }
  .top-right {
    top: 10px;
    right: 10px;
  }
  .bottom-left {
    bottom: 10px;
    left: 10px;
  }
  .bottom-right {
    bottom: 10px;
    right: 10px;
  }

  .loading-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 4px;
    background-color: #f0f0f0;
  }
  .loading-progress {
    height: 100%;
    background-color: #4caf50;
    transition: width 0.3s ease-in-out;
  }
</style>
