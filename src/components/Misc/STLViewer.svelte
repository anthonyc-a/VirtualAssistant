<script lang="ts">
    import { onMount, afterUpdate } from 'svelte';
    import * as THREE from 'three';
    import { STLLoader } from 'three/examples/jsm/loaders/STLLoader';
    import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
    import TGA from 'tga-js';
    
    export let stlPath: string;
    export let texturePaths: {
        diffuse?: string;
        normal?: string;
        ao?: string;
        roughness?: string;
        metalness?: string;
    } = {};
    export let fallbackColor: string = "#ffffff"; // New prop for fallback color
    
    let container: HTMLDivElement;
    let width: number;
    let height: number;
    let renderer: THREE.WebGLRenderer;
    let scene: THREE.Scene;
    let camera: THREE.PerspectiveCamera;
    let controls: OrbitControls;
    let model: THREE.Mesh;
    
    function loadTGA(url: string): Promise<THREE.Texture> {
        return new Promise((resolve, reject) => {
            const loader = new THREE.FileLoader();
            loader.setResponseType('arraybuffer');
            loader.load(
                url,
                function (buffer) {
                    const tga = new TGA();
                    tga.load(new Uint8Array(buffer as ArrayBuffer));
                    const canvas = tga.getCanvas();
                    const texture = new THREE.CanvasTexture(canvas);
                    texture.flipY = false;  // TGA textures are often flipped
                    resolve(texture);
                },
                undefined,
                reject
            );
        });
    }
    
    async function createScene() {
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0x333333);  // Set a gray background for contrast
        
        camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
        camera.position.set(0, 0, 5);
        
        renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(width, height);
        renderer.outputEncoding = THREE.sRGBEncoding;
        container.appendChild(renderer.domElement);
        
        controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        
        const loader = new STLLoader();
        
        try {
            const geometry = await new Promise<THREE.BufferGeometry>((resolve) => loader.load(stlPath, resolve));
    
            const material = new THREE.MeshStandardMaterial({
                color: new THREE.Color(fallbackColor), // Use fallback color initially
                metalness: 0.5,
                roughness: 0.5,
            });
    
            // Load and apply textures
            for (const [key, path] of Object.entries(texturePaths)) {
                if (path) {
                    try {
                        const texture = await loadTGA(path);
                        switch (key) {
                            case 'diffuse': 
                                material.map = texture;
                                material.color.setHex(0xffffff); // Reset color to white when using diffuse texture
                                break;
                            case 'normal': material.normalMap = texture; break;
                            case 'ao': material.aoMap = texture; break;
                            case 'roughness': material.roughnessMap = texture; break;
                            case 'metalness': material.metalnessMap = texture; break;
                        }
                    } catch (error) {
                        console.error(`Failed to load texture: ${key}`, error);
                        // If diffuse texture fails to load, fallback color will be used
                        if (key === 'diffuse') {
                            material.color.set(fallbackColor);
                        }
                    }
                }
            }
    
            material.needsUpdate = true;
    
            model = new THREE.Mesh(geometry, material);
            
            // Center the model
            geometry.computeBoundingBox();
            const center = new THREE.Vector3();
            geometry.boundingBox!.getCenter(center);
            model.position.sub(center);
            
            scene.add(model);
            
            // Adjust camera and controls based on model size
            const box = new THREE.Box3().setFromObject(model);
            const size = box.getSize(new THREE.Vector3());
            const maxDim = Math.max(size.x, size.y, size.z);
            camera.position.z = maxDim * 2;
            controls.target.copy(center);
            controls.update();
        } catch (error) {
            console.error("Error loading model or textures:", error);
        }
        
        // Improved lighting setup
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
    }
    
    function animate() {
        requestAnimationFrame(animate);
        controls.update();
        renderer.render(scene, camera);
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
        }
    }
    
    onMount(() => {
        handleResize();
        createScene();
        animate();
        window.addEventListener('resize', handleResize);
    
        return () => {
            window.removeEventListener('resize', handleResize);
            if (renderer) renderer.dispose();
            if (model) {
                if (model.geometry) model.geometry.dispose();
                if (model.material) {
                    if (Array.isArray(model.material)) {
                        model.material.forEach(m => m.dispose());
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
    
    <div bind:this={container} class="w-full h-[480px]"></div>