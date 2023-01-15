import * as THREE from 'three';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls.js';
import type { Rot3 } from './types';

var loader = new GLTFLoader();
let table: THREE.Group;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1d3161)

const camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 0.1, 1000);

camera.position.y  = 2;

loader.load("/models/schematic_table.gltf", (model) => {
	table = model.scene;
	table.scale.set(1, 1, 1);
	table.position.z = -0.75;
  table.position.y = -0.5;
  table.position.x = -0.5;

  scene.add(model.scene);
}, undefined, (error) => {
  console.error( error );
})

var planeGeometry = new THREE.PlaneGeometry(2, 2);
var planeMaterial = new THREE.MeshBasicMaterial({ color: 0x16264A, side: THREE.DoubleSide });
var plane = new THREE.Mesh(planeGeometry, planeMaterial);
plane.rotation.x = Math.PI / 2;
plane.position.y = -0.5;
plane.rotation.z = Math.PI / 4;
scene.add(plane);

let renderer: any;
camera.position.z = 5;

const color = 0xffffff;
const intensity = 2.5;
const light = new THREE.DirectionalLight(color, intensity);
light.position.set(0, 8, 3);
light.target.position.set(0, 2, 0);
scene.add(light);
scene.add(light.target);

let controls: OrbitControls; 

function easeInQuadratic(t: number) {
  return t * t;
}

const panCameraToPoint = (camera: THREE.PerspectiveCamera, point: THREE.Vector3, angle: Rot3, frames: number, rotationFrameOffset: number) => {
  let stepX = (point.x - camera.position.x) / frames;
  let stepY = (point.y - camera.position.y) / frames;
  let stepZ = (point.z - camera.position.z) / frames;

  let angleStepX = (angle.x - camera.rotation.x) / (frames-rotationFrameOffset);
  let angleStepY = (angle.y - camera.rotation.y) / (frames-rotationFrameOffset);
  let angleStepZ = (angle.z - camera.rotation.z) / (frames-rotationFrameOffset);

  let currentFrame = 0;
  let intervalId = setInterval(() => {
      camera.position.x += stepX;
      camera.position.y += stepY;
      camera.position.z += stepZ;

      if (currentFrame > rotationFrameOffset) {
        camera.rotation.x += angleStepX;
        camera.rotation.y += angleStepY;
        camera.rotation.z += angleStepZ;
      }

      currentFrame++;
      if (currentFrame >= frames) {
          clearInterval(intervalId);
      }
  }, 1000/60);
}

export function panToTable() {
  panCameraToPoint(
    camera,
    new THREE.Vector3(
      camera.position.x,
      camera.position.y,
      -0.2
    ), 
    {
      x: -Math.PI/2,
      y: camera.rotation.y,
      z: camera.rotation.z
    }, 300, 0);
}

const animate = () => {
  requestAnimationFrame(animate);
  renderer.render(scene, camera);
};

const resize = () => {
  renderer.setSize(window.innerWidth, window.innerHeight)
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();

  controls = new OrbitControls(camera, renderer.domElement);
};

export const createScene = (el: any) => {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
  resize();
  animate();
}


window.addEventListener('resize', resize);