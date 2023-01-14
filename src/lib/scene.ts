import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1d3161)

var camera = new THREE.OrthographicCamera(
	-(window.innerWidth),
	(window.innerWidth),
	(window.innerHeight),
	-(window.innerHeight),
	0.1,
	100
  );
camera.position.z = 10;
camera.zoom = 100;

var loader = new GLTFLoader();
let table: THREE.Scene;
let mesh;

loader.load("/models/schematic_table.gltf", (model) => {
	mesh = model.scene;
	mesh.scale.set(5, 5, 5);
	mesh.position.z = 0;
    table = scene.add(model.scene);
}, undefined, (error) => {
    console.error( error );
})

const geometry = new THREE.BoxGeometry(10, 10, 10);

const material = new THREE.MeshStandardMaterial({
	color: 0xff0000,
	metalness: 0.5
});

const cube = new THREE.Mesh(geometry, material);
cube.position.z = -10;
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

	table.rotation.x = Math.PI / 4;
	table.rotation.y = Math.PI / 4;

	renderer.render(scene, camera);
};

const resize = () => {
	renderer.setSize(window.innerWidth, window.innerHeight);
	camera.updateProjectionMatrix();
};

export const createScene = (el: any) => {
	renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
	resize();
	animate();
};

window.addEventListener('resize', resize);