<div
  class="w-full h-full bg-primary border-r-4 border-accent grid grid-cols-13 grid-rows-10 relative"
>
  <!-- Top-left corner -->
  <div
    class="[clip-path:polygon(0_0,0_100%,4px_100%,100%_4px,100%_0)] col-span-1 row-span-1 bg-accent"
  >
    <div
      class="[clip-path:polygon(0_0,100%_0,0_100%)] bg-secondary w-full h-full"
    />
  </div>

  <!-- bottom shape -->
  <!-- right-side -->
  <div
    class="[clip-path:polygon(0_0,100%_calc(100%_-_4px),100%_100%,0_100%)] col-span-1 col-start-12 row-span-1 row-start-10 bg-accent"
  >
    <div
      class="[clip-path:polygon(0_4px,100%_100%,0_100%)] w-full h-full bg-secondary"
    />
  </div>
  <!-- middle part -->
  <div
    class="col-span-5 col-start-7 row-span-1 row-start-10 border-t-4 border-accent bg-secondary"
  />
  <!-- left-side -->
  <div
    class="[clip-path:polygon(0_calc(100%_-_4px),100%_0,100%_100%,0_100%)] col-span-1 col-start-6 row-span-1 row-start-10 bg-accent"
  >
    <div
      class="[clip-path:polygon(0_100%,100%_4px,100%_100%)] w-full h-full bg-secondary"
    />
  </div>

  <!-- Borders -->
  <div class="col-span-12 row-span-1 border-t-4 border-accent" />
  <div class="col-span-1 row-start-2 row-end-10 border-l-4 border-accent" />
  <div
    class="col-span-1 row-span-1 row-start-10 col-start-1 border-l-4 border-b-4 border-accent"
  />
  <div
    class="col-span-1 col-start-13 row-span-1 row-start-10 border-b-4 border-accent"
  />
  <div
    class="col-span-4 col-start-2 row-span-1 row-start-10 border-b-4 border-accent"
  />

  <!-- main content -->
  <div class="col-start-2 row-start-2 col-span-11 row-end-10">
    <slot />
  </div>
</div>
