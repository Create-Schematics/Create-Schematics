import * as THREE from 'three';
import * as TWEEN from '@tweenjs/tween.js'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import type { Rot3 } from './types';
import { writable } from 'svelte/store';

var loader = new GLTFLoader();
let table: THREE.Group;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1d3161)

const camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 0.1, 1000);



camera.lookAt(-0.6, -0.6, -1.6)

camera.position.y = 2;
camera.position.x = 2;
camera.position.z = 5;


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

const color = 0xffffff;
const intensity = 2.5;
const light = new THREE.DirectionalLight(color, intensity);
light.position.set(0, 8, 3);
light.target.position.set(0, 2, 0);
scene.add(light);
scene.add(light.target);

const panCameraToPoint = (camera: THREE.PerspectiveCamera, point: THREE.Vector3, angle: Rot3, newFov: number,  duration: number, resolve: any) => {
  let position = { x: camera.position.x, y: camera.position.y, z: camera.position.z };
  let rotation = { x: camera.rotation.x, y: camera.rotation.y, z: camera.rotation.z };
  let fov = { fov: camera.fov };
  
  let tween = new TWEEN.Tween(position)
      .to({ x: point.x, y: point.y, z: point.z }, duration)
      .onUpdate(() => {
          camera.position.set(position.x, position.y, position.z);
      })
      .start();
  
  let rotationTween = new TWEEN.Tween(rotation)
      .to({ x: angle.x, y: angle.y, z: angle.z }, duration)
      .onUpdate(() => {
          camera.rotation.set(rotation.x, rotation.y, rotation.z);
      })
      .start();

  let zoomTween = new TWEEN.Tween(fov)
    .to({ fov: newFov }, duration)
    .onUpdate(() => {
        camera.fov = fov.fov;
        camera.updateProjectionMatrix();
    })
    .start();

  zoomTween.onComplete(() => {
    resolve();
  })
}

export function panToTable() {
  return new Promise((resolve) => {
    panCameraToPoint(
      camera,
      new THREE.Vector3 (
          0,
          2,
          -0.25
      ), {
        x: -Math.PI/2,
        y: 0,
        z: 0,
      }, 
        25,
        2000,
        resolve
      );
  });
}

const animate = () => {
  requestAnimationFrame(animate);
  TWEEN.update();
  renderer.render(scene, camera);
};

const resize = () => {
  renderer.setSize(window.innerWidth, window.innerHeight)
  camera.aspect = window.innerWidth / window.innerHeight;
  camera.updateProjectionMatrix();
};

export const createScene = (el: any) => {
  renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });
  resize();
  animate();
}


window.addEventListener('resize', resize);