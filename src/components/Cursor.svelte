<script lang="ts">
    import { onMount } from 'svelte';
  
    let x = 0;
    let y = 0;
    let hoverContent = '';
    let cursorState = 'default'; // Can be 'default', 'pointer', or 'text'
  
    function handleMouseMove(event: MouseEvent) {
      x = event.clientX;
      y = event.clientY;
    }
  
    function handleMouseOver(event: MouseEvent) {
      const target = event.target as HTMLElement;
      hoverContent = target.getAttribute('data-cursor-content') || '';
      
      if (target.tagName.toLowerCase() === 'input' || target.tagName.toLowerCase() === 'textarea') {
        cursorState = 'text';
      } else if (window.getComputedStyle(target).cursor === 'pointer') {
        cursorState = 'pointer';
      } else {
        cursorState = 'default';
      }
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
    <div class="cursor-inner" class:pointer={cursorState === 'pointer'} class:text={cursorState === 'text'}></div>
    <div class="cursor-outer" class:pointer={cursorState === 'pointer'} class:text={cursorState === 'text'}></div>
    {#if cursorState === 'text'}
      <div class="cursor-text-line"></div>
    {/if}
    {#if hoverContent}
      <div class="cursor-content">{hoverContent}</div>
    {/if}
  </div>
  
  <style>
    .cursor-container {
      position: fixed;
      pointer-events: none;
      z-index: 99999;
      mix-blend-mode: difference;
    }
  
    .cursor-inner {
      width: 4px;
      height: 4px;
      background-color: white;
      border-radius: 50%;
      position: absolute;
      top: -2px;
      left: -2px;
      transition: all 0.3s ease-out;
    }
  
    .cursor-inner.pointer {
      width: 16px;
      height: 16px;
      top: -8px;
      left: -8px;
    }
  
    .cursor-inner.text {
      width: 2px;
      height: 24px;
      top: -12px;
      left: -1px;
      border-radius: 0;
    }
  
    .cursor-outer {
      width: 32px;
      height: 32px;
      border: 2px solid rgba(255, 255, 255, 0.5);
      border-radius: 50%;
      position: absolute;
      top: -16px;
      left: -16px;
      transition: all 0.3s ease-out;
    }
  
    .cursor-outer.pointer {
      width: 40px;
      height: 40px;
      top: -20px;
      left: -20px;
      background-color: rgba(255, 255, 255, 0.1);
    }
  
    .cursor-outer.text {
      width: 0;
      height: 0;
      opacity: 0;
    }
  
    .cursor-text-line {
      width: 2px;
      height: 24px;
      background-color: white;
      position: absolute;
      top: -12px;
      left: -1px;
      animation: blink 0.7s infinite;
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
  
    @keyframes blink {
      0% { opacity: 0; }
      50% { opacity: 1; }
      100% { opacity: 0; }
    }
  </style>