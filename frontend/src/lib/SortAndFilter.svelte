<script lang="ts">
  import type { Schematic } from "./types";

    let searchTerm = "";
    let selectedCategory = "";
  
    let items: Schematic[];
    function filterItems() {
      return items.filter((item) => {
        // const matchesSearchTerm = item.name.toLowerCase().includes(searchTerm.toLowerCase());
        // const matchesCategory = selectedCategory === "" || item.category === selectedCategory;
        // return matchesSearchTerm && matchesCategory;
      });
    }
  
    function sortItems() {
      // return filterItems().sort((a, b) => a.name.localeCompare(b.name));
    }
  </script>
  
  <div class="text-black">
    <input type="text" id="search" bind:value={searchTerm} />

    <select id="sort" bind:value={selectedCategory}>
        <option value="default" selected>Alphabetical</option>
        <option value="alphabetical">Alphabetical</option>
        <option value="new">New</option>
        <option value="old">Old</option>
        <option value="views">Views</option>
        <option value="downloads">Downloads</option>
        <option value="size">Size</option>
    </select>
  </div>
