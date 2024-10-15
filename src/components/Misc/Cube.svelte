<script lang="ts">
    import { onMount, afterUpdate } from 'svelte';
    import * as THREE from 'three';
    
    let container;
    let width;
    let height;
    let renderer;
    let scene;
    let camera;
    let cube;
    
    function createScene() {
        scene = new THREE.Scene();
        // Scene background is not set, making it transparent
        
        camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
        renderer = new THREE.WebGLRenderer({ 
            alpha: true,
            antialias: true
        });
        
        renderer.setClearColor(0x000000, 0); // The second parameter is the alpha (0 = fully transparent)
        renderer.setSize(width, height);
        container.appendChild(renderer.domElement);
        
        const geometry = new THREE.BoxGeometry();
        const material = new THREE.MeshBasicMaterial({ color: 0xffffff });
        cube = new THREE.Mesh(geometry, material);
        scene.add(cube);
        
        camera.position.z = 2;
    }
    
    function animate() {
        requestAnimationFrame(animate);
        if (cube) {
            cube.rotation.x += 0.01;
            cube.rotation.y += 0.01;
        }
        if (renderer && scene && camera) {
            renderer.render(scene, camera);
        }
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
            if (cube.geometry) cube.geometry.dispose();
            if (cube.material) cube.material.dispose();
        };
    });
    
    afterUpdate(() => {
        handleResize();
    });
    </script>
    
    <div bind:this={container} class="w-full invert dark:invert-0 h-80"></div>