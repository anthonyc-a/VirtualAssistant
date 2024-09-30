<script lang="ts">
  import { onMount } from 'svelte';
  import Heading from "../../components/Heading.svelte";
  import { fade } from 'svelte/transition';

  let name = "";
  let email = "";
  let message = "";
  let files: FileList | null = null;
  let isLoading = false;
  let showConfirmation = false;
  let errors: string[] = [];
  let fileError = "";

  const MAX_FILE_SIZE = 5 * 1024 * 1024; // 5MB
  const MAX_FILES = 3;

  onMount(() => {
    // Any initialization code can go here
  });

  function validateForm() {
    errors = [];
    if (!name.trim()) errors.push("Name is required");
    if (!email.trim()) errors.push("Email is required");
    else if (!/\S+@\S+\.\S+/.test(email)) errors.push("Invalid email format");
    if (!message.trim()) errors.push("Message is required");
    return errors.length === 0;
  }

  function handleFileChange(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files) {
      if (target.files.length > MAX_FILES) {
        fileError = `You can only attach up to ${MAX_FILES} files`;
        files = null;
      } else {
        const invalidFiles = Array.from(target.files).filter((file) => file.size > MAX_FILE_SIZE);
        if (invalidFiles.length > 0) {
          fileError = "One or more files exceed 5MB limit";
          files = null;
        } else {
          files = target.files;
          fileError = "";
        }
      }
    }
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (validateForm()) {
      isLoading = true;
      errors = [];
      const formData = new FormData();
      formData.append('name', name);
      formData.append('email', email);
      formData.append('message', message);
      if (files) {
        Array.from(files).slice(0, MAX_FILES).forEach((file) => {
          formData.append('files', file);
        });
      }

      try {
        const response = await fetch('/api/send-email', {
          method: 'POST',
          body: formData
        });

        const result = await response.json();

        if (result.success) {
          showConfirmation = true;
          name = "";
          email = "";
          message = "";
          files = null;
          setTimeout(() => {
            showConfirmation = false;
          }, 5000);
        } else {
          errors = [result.message || "Failed to send email"];
        }
      } catch (error) {
        console.error("Error sending email:", error);
        errors = ["An error occurred. Please try again."];
      } finally {
        isLoading = false;
      }
    }
  }
</script>

<div class="max-w-2xl w-full mx-auto">
  <Heading
    heading="Get in Touch"
    subheading="Embracing an iterative approach to design and development."
    showBreak={false}
  />

  <div class="max-w-2xl mx-auto mt-6">
    <form class="p-6 border border-border rounded-3xl" on:submit={handleSubmit}>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="name">Name</label>
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="name"
          type="text"
          bind:value={name}
          placeholder="Your name"
          required
        />
      </div>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="email">Email</label>
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="email"
          type="email"
          bind:value={email}
          placeholder="Your email"
          required
        />
      </div>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="message">Message</label>
        <textarea
          class="shadow appearance-none border rounded w-full py-2 px-3 text-foreground leading-tight focus:outline-none focus:shadow-outline"
          id="message"
          bind:value={message}
          placeholder="Your message"
          rows="5"
          required
        ></textarea>
      </div>
      <div class="mb-4">
        <label class="block text-foreground text-sm font-bold mb-2" for="files">Attachments (optional, max 3 files, 5MB each)</label>
        <input
          type="file"
          id="files"
          on:change={handleFileChange}
          class="w-full text-sm text-foreground file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-accent file:text-white hover:file:bg-accent-dark"
          multiple
          accept=".pdf,.doc,.docx,.txt,.jpg,.jpeg,.png"
        />
        {#if fileError}
          <p class="text-red-500 text-xs italic mt-1">{fileError}</p>
        {/if}
      </div>
      {#if errors.length > 0}
        <div class="mb-4 bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
          <ul class="list-disc list-inside">
            {#each errors as error}
              <li>{error}</li>
            {/each}
          </ul>
        </div>
      {/if}
      <div class="flex items-center justify-between">
        <button
          class="bg-accent w-full hover:bg-accent-dark text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50"
          type="submit"
          disabled={isLoading}
        >
          {isLoading ? 'Sending...' : 'Send'}
        </button>
      </div>
    </form>
  </div>
</div>

{#if showConfirmation}
  <div transition:fade="{{ duration: 300 }}" class="fixed bottom-4 right-4 bg-green-500 text-white p-4 rounded-md shadow-lg">
    Message sent successfully!
  </div>
{/if}