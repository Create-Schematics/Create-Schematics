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

export default class Scene {
	public isOverTable: boolean = false;
	public shouldAnimate: boolean = true;

	public scene: THREE.Scene;
	public camera: THREE.PerspectiveCamera;
	public renderer: THREE.WebGLRenderer;
	public raycaster: THREE.Raycaster;

	public table = new THREE.Group;
	public button = new THREE.Mesh;

	public mouse: THREE.Vector2 = new THREE.Vector2;

	constructor(el: any) {
		this.scene = new THREE.Scene();

		this.renderer = new THREE.WebGLRenderer({ antialias: true, canvas: el });

		this.camera = new THREE.PerspectiveCamera(35, window.innerWidth / window.innerHeight, 0.1, 1000);

		this.camera.lookAt(-0.6, -0.6, -1.6)
		this.camera.position.set(2, 2, 5);

		this.raycaster = new THREE.Raycaster();

		this.loadSchematicTable();
		this.loadRenderedButton();

		this.initialisePlane();
		this.initialiseLights();
	}

	public animate(): void {
		if (!this.shouldAnimate) return;
		requestAnimationFrame(this.animate);
		if (this.button) this.button.lookAt(this.camera.position);
		TWEEN.update();
		this.renderer.render(this.scene, this.camera);
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

	private async loadGLTFfile(path: string): Promise<THREE.Group> {
		return new Promise((resolve) => {
			var loader = new GLTFLoader();
			loader.load(path, (model) => {
				resolve(model.scene)
			}, undefined, (error) => {
				console.error(error);
			});
		});
	}

	private async loadSchematicTable(): Promise<void> {
		this.table = await this.loadGLTFfile("/models/schematic_table.gltf");

		this.table.scale.set(1, 1, 1);

		this.table.position.z = -0.75;
		this.table.position.y = -0.5;
		this.table.position.x = -0.5;

		this.scene.add(this.table);
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
		this.scene.background = new THREE.Color(0x1d3161)

		const color = 0xffffff;
		const intensity = 2.5;
		const light = new THREE.DirectionalLight(color, intensity);
		const ambientLight = new THREE.AmbientLight(color, intensity/8.5);

		light.position.set(0, 8, 3);
		light.target.position.set(0, 2, 0);

		this.scene.add(light);
		this.scene.add(light.target);

		this.scene.add(ambientLight);
	}
	
	public async panToTable(): Promise<void> {
		await panCameraToPoint(
			this.camera,
			new THREE.Vector3 (0, 2, -0.25),
			{ x: -Math.PI/2, y: 0, z: 0, }, 
			30, 2000
		);
	}
	
	public onClickEvent(event: any): void {
		this.mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
		this.mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

		this.raycaster.setFromCamera(this.mouse, this.camera);
		const intersects = this.raycaster.intersectObjects([this.button, this.table]);
		alert("Intersect")
		if (intersects.length > 0) {
			alert("Intersect")
			this.panToTable();
			this.isOverTable = !this.isOverTable;
		}
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
