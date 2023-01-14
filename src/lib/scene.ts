import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1d3161)

const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
camera.position.z = 5;

const geometry = new THREE.BoxGeometry();

const material = new THREE.MeshStandardMaterial({
	color: 0xff0000,
	metalness: 0.5
});

var loader = new GLTFLoader();
let table: THREE.Scene;
let mesh;

loader.load("/models/schematic_table.gltf", (model) => {
	mesh = model.scene;
	mesh.scale.set(2, 2, 2);
    table = scene.add(model.scene);
}, undefined, (error) => {
    console.error( error );
})


const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

const directionalLight = new THREE.DirectionalLight(0x9090aa);
directionalLight.position.set(-10, 10, -10).normalize();
scene.add(directionalLight);

const hemisphereLight = new THREE.HemisphereLight(0xffffff, 0xffffff);
hemisphereLight.position.set(2, 0, 0);
scene.add(hemisphereLight);

let renderer: any;

const animate = () => {
	requestAnimationFrame(animate);
	cube.rotation.x += 0.01;
	cube.rotation.y += 0.01;

	renderer.render(scene, camera);
};

const resize = () => {
	renderer.setSize(window.innerWidth, window.innerHeight);
	camera.aspect = window.innerWidth / window.innerHeight;
	camera.updateProjectionMatrix();
};

export const createScene = (el: any) => {
	renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
	resize();
	animate();
};

window.addEventListener('resize', resize);