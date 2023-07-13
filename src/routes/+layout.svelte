<script>
  import "../app.postcss";
  import { DarkMode } from "flowbite-svelte";
</script>

<body class="dark:bg-gray-800 w-screen h-screen">
  <DarkMode />
  <h1 class="text-white">test</h1>
  <slot />
</body>
