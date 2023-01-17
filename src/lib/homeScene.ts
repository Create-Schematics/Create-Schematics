import * as THREE from 'three';
import * as TWEEN from '@tweenjs/tween.js'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import type { Rot3 } from './types';
import TextGeometry from 'three-text-geometry';
import { FontLoader, Font } from 'three/examples/jsm/loaders/FontLoader.js';

var loader = new GLTFLoader();
let table: THREE.Group;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x1d3161)

const mouse = new THREE.Vector2();
const camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 0.1, 1000);
var raycaster = new THREE.Raycaster();

const textureLoader = new THREE.TextureLoader();
const texture = textureLoader.load('/home/button-outline.png');

var buttonGeometery = new THREE.PlaneGeometry(0.1, 0.1);
var buttonMaterial = new THREE.MeshStandardMaterial({ 
  map: texture, 
  transparent: true
});



var button = new THREE.Mesh(buttonGeometery, buttonMaterial);
scene.add(button)

button.position.x = 0.15;
button.position.y = 1;
button.position.z = 0.25;

const fontLoader = new FontLoader();

fontLoader.load('/fonts/MinecraftRegular.json', (font: Font) => {
  const textGeometry = new TextGeometry( 'Hello, World!', {
      font: font,
      size: 1,
      height: 0.1,
      curveSegments: 12,
      bevelEnabled: true,
      bevelThickness: 0.1,
      bevelSize: 0.1,
      bevelSegments: 1
    }
  );

  const material = new THREE.MeshStandardMaterial({ color: 0xffffff });

  const mesh = new THREE.Mesh(textGeometry, material);

  scene.add(mesh);
})

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

function onclick(event: any) {
  mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
  mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;
  raycaster.setFromCamera(mouse, camera);
  const intersects = raycaster.intersectObjects([button, table]);
  if (intersects.length > 0) {
    panToTable();
  }
}

const animate = () => {
  requestAnimationFrame(animate);
  button.lookAt(camera.position);
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
  renderer.domElement.addEventListener("click", onclick, true);
  resize();
  animate();
}

window.addEventListener('resize', resize);