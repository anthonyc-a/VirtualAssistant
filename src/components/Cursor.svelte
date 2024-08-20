<script lang="ts">
    import { onMount } from 'svelte';
  
    let x = 0;
    let y = 0;
    let hoverContent = '';
    let isPointer = false;
  
    function handleMouseMove(event: MouseEvent) {
      x = event.clientX;
      y = event.clientY;
    }
  
    function handleMouseOver(event: MouseEvent) {
      const target = event.target as HTMLElement;
      hoverContent = target.getAttribute('data-cursor-content') || '';
      isPointer = window.getComputedStyle(target).cursor === 'pointer';
    }
  
    onMount(() => {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseover', handleMouseOver);
  
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseover', handleMouseOver);
      };
    });
  </script>
  
  <div class="cursor-container" style="left: {x}px; top: {y}px">
    <div class="cursor-inner" class:pointer={isPointer}></div>
    <div class="cursor-outer" class:pointer={isPointer}></div>
    {#if hoverContent}
      <div class="cursor-content">{hoverContent}</div>
    {/if}
  </div>
  
  <style>
    :global(body) {
      cursor: none !important;
    }
  
    .cursor-container {
      position: fixed;
      pointer-events: none;
      z-index: 9999;
    }
  
    .cursor-inner {
      width: 8px;
      height: 8px;
      background-color: white;
      border-radius: 50%;
      position: absolute;
      top: -4px;
      left: -4px;
      transition: width 0.2s, height 0.2s, top 0.2s, left 0.2s;
    }
  
    .cursor-inner.pointer {
      width: 16px;
      height: 16px;
      top: -8px;
      left: -8px;
    }
  
    .cursor-outer {
      width: 32px;
      height: 32px;
      border: 2px solid rgba(255, 255, 255, 0.5);
      border-radius: 50%;
      position: absolute;
      top: -16px;
      left: -16px;
      transition: all 0.1s ease-out;
    }
  
    .cursor-outer.pointer {
      width: 40px;
      height: 40px;
      top: -20px;
      left: -20px;
      background-color: rgba(255, 255, 255, 0.1);
    }
  
    .cursor-content {
      position: absolute;
      top: 24px;
      left: 24px;
      background-color: rgba(0, 0, 0, 0.8);
      color: white;
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 12px;
      white-space: nowrap;
    }
  </style>