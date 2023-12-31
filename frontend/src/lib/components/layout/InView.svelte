<script lang="ts">
    import { onMount } from "svelte";

    // By default only play the animation the first time the element is scrolled by
    export let once = true;

    let intersects = false;
    let box: HTMLElement;

    const observe = (): () => void => {
        const observer = new IntersectionObserver(entries => {
            intersects = entries[0].isIntersecting;

            if (intersects && once) {
                observer.unobserve(box);
            }
        })
        
        observer.observe(box);

        return () => observer.unobserve(box);
    }

    // Fallback incase the intersection observer api isnt available
    const bounding = (): () => void => {
        const c = box.getBoundingClientRect();
        intersects = c.top < window.innerHeight && c.bottom > 0;
                
        if (intersects && once) {
            window.removeEventListener("scroll", bounding);
        }
        
        window.addEventListener("scroll", bounding);
        return () => window.removeEventListener("scroll", bounding);
    }

    onMount(() => {
        // The interaction observer isnt supported in some browser so provide
        // a fallback implementation, could look into using a polyfill isntead
        return IntersectionObserver ? observe() : bounding();
    })
</script> 

<div bind:this={box}>
    {#if intersects}
        <div>
            <slot/>
        </div>
    {:else}
        <div class="opacity-0">
            <slot/>
        </div>
    {/if}
</div>