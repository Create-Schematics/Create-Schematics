import * as TWEEN from '@tweenjs/tween.js'
import * as THREE from 'three';
import type { Rot3 } from './types';
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'

function panCameraToPoint( camera: THREE.PerspectiveCamera, point: THREE.Vector3, angle: Rot3, newFov: number, duration: number): Promise<void> {
	return new Promise((resolve) => {
		let position = {
			x: camera.position.x,
			y: camera.position.y,
			z: camera.position.z 
		};
		let rotation = {
			x: camera.rotation.x, 
			y: camera.rotation.y, 
			z: camera.rotation.z 
		};

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
	});
  }

class Scene {
	private isOverTable: boolean = false;

	private scene: THREE.Scene;
	private camera: THREE.PerspectiveCamera;
	private renderer: THREE.WebGLRenderer;
	private raycaster: THREE.Raycaster;

	private table: THREE.Group | undefined;
	private button: THREE.Mesh | undefined;

	private mouse: THREE.Vector2 = new THREE.Vector2;

	private animate() {
		requestAnimationFrame(this.animate);
		if (this.button) this.button.lookAt(this.camera.position);
		TWEEN.update();
		this.renderer.render(this.scene, this.camera);
	}

	constructor(el: any) {
		this.scene = new THREE.Scene();
		this.camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 0.1, 1000);
		this.raycaster = new THREE.Raycaster();

		this.renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });

		this.loadSchematicTable();
		this.loadRenderedButton();

		this.initialisePlane();
		this.initialiseLights();

		this.resize();
		this.animate();

		window.addEventListener('resize', this.resize);
		this.renderer.domElement.addEventListener("click", this.onClickEvent, true);
	}

	private loadRenderedButton(): void {
		const textureLoader = new THREE.TextureLoader();
		const texture = textureLoader.load('/home/button-outline.png');

		var buttonGeometery = new THREE.PlaneGeometry(0.1, 0.1);

		var buttonMaterial = new THREE.MeshStandardMaterial({ 
			map: texture, 
			transparent: true
		});

		this.button = new THREE.Mesh(buttonGeometery, buttonMaterial);
		this.scene.add(this.button)

		this.button.position.x = 0.15;
		this.button.position.y = 1;
		this.button.position.z = 0.25;
	}

	private loadSchematicTable(): void {
		var loader = new GLTFLoader();

		loader.load("/models/schematic_table.gltf", (model) => {
			this.table = model.scene;
			this.table.scale.set(1, 1, 1);

			this.table.position.z = -0.75;
			this.table.position.y = -0.5;
			this.table.position.x = -0.5;
		  	this.scene.add(model.scene);
		}, undefined, (error) => {
		  	console.error( error );
		})
	}

	private initialisePlane(): void {
		var planeGeometry = new THREE.PlaneGeometry(2, 2);
		var planeMaterial = new THREE.MeshBasicMaterial({ color: 0x16264A, side: THREE.DoubleSide });
		var plane = new THREE.Mesh(planeGeometry, planeMaterial);
		
		plane.rotation.x = Math.PI / 2;
		plane.position.y = -0.5;
		plane.rotation.z = Math.PI / 4;

		this.scene.add(plane);
	}

	private initialiseLights(): void {
		const color = 0xffffff;
		const intensity = 2.5;
		const light = new THREE.DirectionalLight(color, intensity);

		light.position.set(0, 8, 3);
		light.target.position.set(0, 2, 0);

		this.scene.add(light);
		this.scene.add(light.target);
	}
	
	public async panToTable(): Promise<void> {
		await panCameraToPoint(
			this.camera,
			new THREE.Vector3 (0, 2, -0.25),
			{ x: -Math.PI/2, y: 0, z: 0, }, 
			25, 2000
		);
	}
	
	private onClickEvent(event: any): void {
		if (!(this.button && this.table))
			return;

		this.mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
		this.mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

		this.raycaster.setFromCamera(this.mouse, this.camera);
		const intersects = this.raycaster.intersectObjects([this.button, this.table]);

		if (intersects.length > 0)
			this.panToTable();
			this.isOverTable = !this.isOverTable;
	}

	private resize(): void {
		this.renderer.setSize(window.innerWidth, window.innerHeight)
		this.camera.aspect = window.innerWidth / window.innerHeight;
		this.camera.updateProjectionMatrix();
	}

	public getCamera(): THREE.PerspectiveCamera {
		return this.camera;
	}
}
