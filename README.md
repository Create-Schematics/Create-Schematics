# Create Schematics (Rewritten)
Our Related Projects [ <a href="https://github.com/Rabbitminers/Create-Schematics-Backend"> Backend </a> ], [ <a href="https://github.com/Rabbitminers/Create-Schematics-Companion"> Companion Mod </a> ] <br/>
View The Site - createschematics.com [Currently shows coming soon page]

<hr/>

### Project Information

Create schematics front-end is built with Sveltekit and Three.js with Typescript. For a more detailed explanation on how things work in general, check out their respective docs <a href="https://svelte.dev/tutorial/basics">Svelte</a>, <a href="https://kit.svelte.dev/docs/introduction">SvelteKit</a>, <a href="https://threejs.org/docs/index.html#manual/en/introduction/Creating-a-scene">Three.js</a>.
<hr/>

### Development
Once you've cloned the project and installed dependencies with npm install (or pnpm install or yarn), start a development server:

```
# start the server and open the app in a new browser tab
npm run dev -- --open
```

Authentication is handled by firebase, if you need this for testing, create a local env file (`.env.local`) in the root of the directory with all of these fields filled with you're own firebase credentials

```
PUBLIC_FIREBASE_API_KEY=<FIREBASE_API_KEY>
PUBLIC_FIREBASE_AUTH_DOMAIN=<FIREBASE_AUTH_DOMAIN>
PUBLIC_FIREBASE_PROJECT_ID=<FIREBASE_PROJECT_ID>
PUBLIC_FIREBASE_STORAGE_BUCKET=<FIREBASE_STORAGE_BUCKET>
PUBLIC_FIREBASE_MESSAGING_SENDER_ID=<FIREBASE_MESSAGING_SENDER_ID>
PUBLIC_FIREBASE_APP_ID=<FIREBASE_APP_ID>
```

<hr/>

### License

Create Schematics is licensed under CC-BY-NC-ND (Creative Commons Non-Commercial No-Derivatives License): You may copy and redistribute the material in any medium or format. However, the material may not be used for commercial purposes and if you remix, transform or build upon the material these modifications cannot be distributed.