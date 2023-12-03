<script lang="ts">
</script>
<svelte:head>
    <title>API Docs - Create Schematics</title>
    <meta name="description" content="SwagerUI Api Documentation for the Create Schematics REST API"/>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
    <!-- theme from https://github.com/ostranme/swagger-ui-themes under MIT license -->
    <link rel="stylesheet" href="path/to/swagger-ui-themes/css/theme-flattop.css" />
</svelte:head>
<main>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin="anonymous"></script>
    <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-standalone-preset.js" crossorigin="anonymous"></script>
    <script>
      window.onload = () => {
        window.ui = SwaggerUIBundle({
          url: '../../openapi.json',
          dom_id: '#swagger-ui',
          presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
          ],
          layout: "StandaloneLayout",
        });
      };
    </script>
</main>