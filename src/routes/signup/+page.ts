// Prevent server calling homeScene.ts resulting in crash as window is undefined
export const ssr = false;
export const prerender = true;