<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import hljs from "highlight.js/lib/core";
  import rust from "highlight.js/lib/languages/rust";
  import "highlight.js/styles/default.css"; // You can choose a different style

  hljs.registerLanguage("rust", rust);

  export let message: string;

  let response: string | null = null;
  let displayedResponse: Array<{
    type: "text" | "code";
    content: string;
    language?: string;
  }> = [];
  let error: string | null = null;
  let loading: boolean = false;

  async function handleSubmit() {
    loading = true;
    error = null;
    response = null;
    displayedResponse = [];

    try {
      const result: any = await invoke("send_message_to_anthropic", {
        message,
      });
      const parsedResult = JSON.parse(result);
      response = parsedResult.content[0].text;
      typeResponse(response);
    } catch (err) {
      error = err instanceof Error ? err.message : "An unknown error occurred";
    } finally {
      loading = false;
      message = "";
    }
  }

  function typeResponse(text: string) {
    const segments = splitIntoSegments(text);
    let currentSegmentIndex = 0;
    let currentCharIndex = 0;

    const typingInterval = setInterval(() => {
      if (currentSegmentIndex < segments.length) {
        const segment = segments[currentSegmentIndex];
        if (currentCharIndex < segment.content.length) {
          if (currentCharIndex === 0) {
            displayedResponse = [
              ...displayedResponse,
              { ...segment, content: "" },
            ];
          }
          displayedResponse[currentSegmentIndex].content +=
            segment.content[currentCharIndex];
          currentCharIndex++;
          displayedResponse = [...displayedResponse];
        } else {
          currentSegmentIndex++;
          currentCharIndex = 0;
        }
      } else {
        clearInterval(typingInterval);
        highlightCode();
      }
    }, 20); // Adjust this number to change typing speed
  }

  function splitIntoSegments(text: string) {
    const segments = [];
    const codeBlockRegex = /```(\w+)?\s*([\s\S]*?)```/g;
    let lastIndex = 0;
    let match;

    while ((match = codeBlockRegex.exec(text)) !== null) {
      if (match.index > lastIndex) {
        segments.push({
          type: "text",
          content: text.slice(lastIndex, match.index),
        });
      }
      segments.push({
        type: "code",
        language: match[1] || "plaintext",
        content: match[2].trim(),
      });
      lastIndex = match.index + match[0].length;
    }

    if (lastIndex < text.length) {
      segments.push({ type: "text", content: text.slice(lastIndex) });
    }

    return segments;
  }

  function highlightCode() {
    const codeBlocks = document.querySelectorAll("pre code");
    codeBlocks.forEach((block) => {
      hljs.highlightBlock(block);
    });
  }

  onMount(() => {
    highlightCode();
  });
</script>

{#if loading}
  <p class="text-gray-600">Loading...</p>
{:else if error}
  <p class="error text-red-500">Error: {error}</p>
{:else if response}
  {#each displayedResponse as segment}
    {#if segment.type === "text"}
      <p class="whitespace-pre-wrap">{segment.content}</p>
    {:else if segment.type === "code"}
      <pre><code class="language-{segment.language}">{segment.content}</code
        ></pre>
    {/if}
  {/each}
{/if}
<form on:submit|preventDefault={handleSubmit} class="mb-4">
  <button
    type="submit"
    disabled={loading || !message}
    class="mt-4 px-4 py-2 font-medium bg-white text-black ml-auto rounded-full w-1/3 hover:bg-blue-600 disabled:opacity-50"
  >
    Submit
  </button>
</form>

<style>
  pre {
    background-color: #151515;
    padding: 1em;
    border-radius: 4px;
    overflow-x: auto;
  }
</style>
