<script>
  import "../app.postcss";
  import { DarkMode } from "flowbite-svelte";
</script>

<body class="dark:bg-gray-800 w-screen h-screen">
  <DarkMode />
  <slot />
</body>
